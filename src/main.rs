
use async_std::sync::Arc;
use env_logger::Builder;
use iced::{Application, Command, Element, executor, Theme, window};
use iced::futures::lock::Mutex;
use iced::Length::{Fill};
use iced::widget::{
    Container, Text,
};
use iced::window::icon::Icon;
use linked_hash_set::LinkedHashSet;
use log::{debug, info, LevelFilter};
use tonic::transport::Channel;

use crate::assets::logo_bytes;
use crate::frontend::{EditTarget, LoginResult, TabId};
use crate::frontend::sims_ims_frontend::sims_frontend_client::SimsFrontendClient;
use crate::states::SimsClientState;
use crate::ui_messages::Message;
use crate::ui_messages::Message::{StartEditing, StopEditing};

mod assets;
mod frontend;
mod ui_messages;
mod states;
mod styles;
mod views;

const SERVER_ADDRESS: &str = "http://localhost:50051";

pub fn main() -> iced::Result {
    Builder::new()
        .filter_module("cs4471_sims_cli_client", LevelFilter::Debug)
        .init();
    ClientState::run(iced::Settings {
        window: window::Settings {
            icon: match logo_bytes() {
                None => None,
                Some((bytes, width, height)) => Icon::from_rgba(bytes, width, height).ok(),
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
    tabs: LinkedHashSet<TabId>,
    edit_item: Option<EditTarget>,
}

impl Application for ClientState {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    
    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut new_client = ClientState {
            username: String::new(),
            state: SimsClientState::Unauthenticated {
                password: "".to_string(),
                error_message: None,
            },
            rpc: Arc::new(Mutex::new(None)),
            token: None,
            current_tab: Vec::new(),
            tabs: LinkedHashSet::new(),
            edit_item: None,
        };

        new_client.tabs.insert(TabId::AllShelves);
        new_client.tabs.insert(TabId::AllItems);
        (new_client, Command::none())
    }

    fn title(&self) -> String {
        "SIMS Inventory Management System".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::UsernameInputChanged(s) => {
                if let SimsClientState::Unauthenticated { .. } = self.state {
                    self.username = s
                }

                Command::none()
            }
            Message::PasswordInputChanged(s) => {
                if let SimsClientState::Unauthenticated {
                    ref mut password, ..
                } = self.state
                {
                    password.clear();
                    password.push_str(&s)
                }

                Command::none()
            }
            Message::LoginButtonClicked => {
                if let SimsClientState::Unauthenticated { ref password, .. } = self.state {
                    let client_ = Arc::clone(&self.rpc);
                    let ret = Command::perform(
                        frontend::login(
                            client_,
                            SERVER_ADDRESS.to_owned(),
                            self.username.to_owned(),
                            password.to_owned(),
                        ),
                        Message::Authenticated,
                    );
                    self.state = SimsClientState::Authenticating;
                    ret
                } else {
                    // login button clicked in non-login screen
                    Command::none()
                }
            }
            Message::Authenticated(authentication_result) => {
                match authentication_result {
                    Ok(response) => {
                        debug!("token received: {}", response.token);
                        self.token = Some(response.token);

                        if matches!(self.state, SimsClientState::Authenticating { .. }) {
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
                                LoginResult::NotConnected => {
                                    "Could not connect to server".to_owned()
                                }
                            }),
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
                    TabId::ShelfView(_) => {
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
            StopEditing => {
                self.edit_item = None;
                Command::none()
            }
            StartEditing(target) => {
                self.edit_item = Some(target);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        match &self.state {
            SimsClientState::Unauthenticated {
                password,
                error_message,
            } => views::unauthenticated_view(self, password, error_message),
            SimsClientState::Authenticating { .. } => Container::new("Logging In...")
                .width(Fill)
                .height(Fill)
                .center_x()
                .center_y()
                .into(),
            SimsClientState::InventoryView => views::inventory_view(self),
            _ => Container::new(Text::new(format!(
                "Placeholder for state: {:?}",
                self.state
            )))
            .width(Fill)
            .height(Fill)
            .center_x()
            .center_y()
            .into(),
        }
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
