use std::io::Cursor;
use async_std::sync::{Arc};
use env_logger::Builder;
use iced::{Color, Command, executor, Length, Rule, window};
use iced::pure::{Application, Element};
use iced::futures::lock::Mutex;
use iced::Length::{Fill, Shrink};
use iced::pure::widget::{button, Container, Text, Image, Space, TextInput, Column, Button, Svg, image as iced_image, Row};
use iced::window::icon::Icon;
use iced::widget::svg::Handle;
use iced_aw::pure::Modal;
use image::ImageFormat;
use linked_hash_set::LinkedHashSet;
use log::{debug, info, LevelFilter};
use image::io::Reader as ImageReader;
use tonic::transport::{Channel};
use crate::assets::{logo_bytes, SIMS_LOGO_SQUARE};
use crate::frontend::{LoginResult, create_tab, TabId};

use crate::frontend::sims_ims_frontend::sims_frontend_client::SimsFrontendClient;
use crate::iced_messages::Message;
use crate::states::SimsClientState;

mod states;
mod frontend;
mod iced_messages;
mod assets;
mod styles;

const SERVER_ADDRESS: &str = "http://localhost:50051";

pub fn main() -> iced::Result {
    Builder::new()
        .filter_module("cs4471_sims_cli_client", LevelFilter::Debug)
        .init();
    ClientState::run(iced::Settings{
        window: window::Settings{
            icon: match logo_bytes() {
                None => None,
                Some((bytes, width, height)) => {
                    Icon::from_rgba(bytes, width, height).ok()
                }
            },
            ..window::Settings::default()
        },
        ..iced::Settings::default()
    })
}


#[derive(Debug)]
struct ClientState {
    username: String,
    state: SimsClientState,
    rpc: Arc<Mutex<Option<SimsFrontendClient<Channel>>>>,
    token: Option<String>,
    current_tab: Vec<TabId>,
    tabs: LinkedHashSet<TabId>
}

impl Application for ClientState {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>)  {
        let mut new_client = ClientState {
            username: String::new(),
            state: SimsClientState::Unauthenticated{
                password: "".to_string(),
                error_message: None
            },
            rpc: Arc::new(Mutex::new(None)),
            token: None,
            current_tab: Vec::new(),
            tabs: LinkedHashSet::new()
        };

        new_client.tabs.insert(
            TabId::AllShelves
        );
        new_client.tabs.insert(
            TabId::AllItems
        );
        (
            new_client,
            Command::none()
        )
    }

    fn title(&self) -> String {
        "SIMS Inventory Management System".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::UsernameInputChanged(s) => {
                if let SimsClientState::Unauthenticated{..} = self.state {
                    self.username = s
                }

                Command::none()
            },
            Message::PasswordInputChanged(s) => {
                if let SimsClientState::Unauthenticated{ref mut password, .. } = self.state {
                    password.clear();
                    password.push_str(&s)
                }

                Command::none()
            },
            Message::LoginButtonClicked => {
                if let SimsClientState::Unauthenticated { ref password, .. } = self.state {
                    let client_ = Arc::clone(&self.rpc);
                    let ret = Command::perform(frontend::login(client_, SERVER_ADDRESS.to_owned(), self.username.to_owned(), password.to_owned()), Message::Authenticated);
                    self.state = SimsClientState::Authenticating;
                    ret
                }else {
                    // login button clicked in non-login screen
                    Command::none()
                }
            }
            Message::Authenticated(authentication_result) => {
                match authentication_result {
                    Ok(response) => {
                        debug!("token received: {}", response.token);
                        self.token = Some(response.token);

                        if matches!(self.state, SimsClientState::Authenticating{..}) {
                            self.state = SimsClientState::InventoryView;
                        }
                        Command::none()
                    }
                    Err(err) => {
                        info!("Failed to log in {:?}", err);
                        self.username = String::new();
                        self.state = SimsClientState::Unauthenticated {
                            password: String::new(),
                            error_message: Some(match err {
                                LoginResult::ServerError(e) => format!("Placeholder: {:?}", e), //TODO: fill in text
                                LoginResult::NotConnected => "Could not connect to server".to_owned()
                            })
                        };
                        Command::none()
                    }
                }
            }
            Message::TabSelected(tab_id) => {
                debug!("Selected tab {:?}", tab_id);
                self.current_tab.push(tab_id);
                Command::none()
            }
            Message::CloseShelf(tab_id) => {
                match tab_id {
                    TabId::AllShelves | TabId::AllItems => {} // can't delete these tabs
                    TabId::ShelfItems(_) => {
                        self.tabs.remove(&tab_id);
                        // replace with drain_filter when stable
                        for i in (0..self.current_tab.len()).rev() {
                            if self.current_tab[i] == tab_id {
                                self.current_tab.remove(i);
                            }
                        }
                    }
                }

                Command::none()
            }
            Message::OpenShelf(tab_id) => {
                if !self.tabs.contains(&tab_id) {
                    self.tabs.insert(tab_id.clone());
                    self.current_tab.push(tab_id); // there are n + 2 tabs (all shelves and all items)
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        match &self.state {
            SimsClientState::Unauthenticated { password, error_message} => {
                let elements = Row::new()
                    .push(Space::with_width(Length::FillPortion(3)))
                    .push(
                        Column::new()
                            .push(Container::new(Image::new(iced_image::Handle::from_memory(Vec::from(assets::SIMS_LOGO_SQUARE))).height(Length::Units(110))).width(Fill).center_x())
                            .push(Container::new(Text::new("SIMS IMS").size(30)).width(Fill).center_x())
                            .push(Rule::horizontal(20))
                            .push(TextInput::new("Username", &self.username, Message::UsernameInputChanged).padding(10))
                            .push(TextInput::new("Password", password, Message::PasswordInputChanged).padding(10).password())
                            .push(Rule::horizontal(20))
                            .push(Button::new("Login").on_press(Message::LoginButtonClicked).width(Fill))
                            .push(Container::new(Text::new(match error_message{Some(message)=>message, None=>""}).color(
                            Color::from_rgba(1.0, 0.0, 0.0, 1.0)
                        )).height(Length::Units(50)).center_x().center_y()
                            )
                            .width(Length::FillPortion(2)))
                    .push(Space::with_width(Length::FillPortion(3)));

                Container::new(elements)
                    .width(Fill)
                    .height(Fill)
                    .center_x()
                    .center_y()
                    .into()
            },
            SimsClientState::Authenticating {..} => {
                Container::new("Logging In...")
                    .width(Fill)
                    .height(Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
            SimsClientState::InventoryView  => {
                let page_content: Element<'_, Self::Message> = match self.current_tab.last().unwrap_or_default() {
                    TabId::AllShelves => Row::new()
                        .push(Text::new("All shelves view"))
                        .push(Button::new("Meep").on_press(Message::OpenShelf(TabId::ShelfItems("shelf0".to_owned()))))
                        .push(Button::new("Meep2").on_press(Message::OpenShelf(TabId::ShelfItems("shelf1".to_owned()))))
                        .into(),
                    TabId::AllItems => Text::new("All items view").into(),
                    TabId::ShelfItems (shelf_id) => {
                        let text_content = format!("Shelf Items view for shelf {}", shelf_id);
                        Modal::new(true, Text::new(text_content), ||{Text::new("Modal!").into()}).into()
                    }
                };

                let tabs = self.tabs.iter()
                    .map(| tab_info| match tab_info {
                        TabId::AllShelves => create_tab(tab_info.clone(), "Shelves".to_owned(), false, Some('\u{F685}')),
                        TabId::AllItems => create_tab(tab_info.clone(), "Items".to_owned(), false, Some('\u{F7D3}')),
                        TabId::ShelfItems(shelf_id) => {
                            create_tab(tab_info.clone(), shelf_id.clone(), true, Some('\u{F1C8}'))
                        }
                    })
                    .fold(
                        Row::new(),
                        |tabs_container, tab|{
                            tabs_container.push(Space::with_width(Length::Units(2))).push(tab)
                        }
                    );

                Column::new()
                    .push(
                    Container::new(
                            tabs
                        )
                        .width(Fill)
                        .height(Shrink)
                        .padding(5))
                    .push(
                        Container::new(page_content)
                        .width(Fill)
                        .height(Fill)
                    ).into()
            }
            _ => Container::new(Text::new(format!("Placeholder for state: {:?}", self.state))).width(Fill).height(Fill).center_x().center_y().into()
        }
    }
}
