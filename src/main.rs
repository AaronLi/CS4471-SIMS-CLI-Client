use std::collections::HashMap;
use async_std::sync::Arc;
use env_logger::Builder;
use std::env::set_var;
use iced::{Application, Command, Element, executor, Theme, window};
use iced::futures::lock::Mutex;
use iced::futures::TryFutureExt;
use iced::Length::{Fill};
use iced::widget::{
    Container, Text,
};
use iced::window::icon::Icon;
use linked_hash_set::LinkedHashSet;
use log::{debug, error, info, LevelFilter};
use tonic::transport::Channel;
use std::future::IntoFuture;

use crate::assets::logo_bytes;
use crate::frontend::{create_item, create_shelf, EditTarget, GetItemsResponse, LoginResult, read_items, read_shelves, RpcCallResult, TabId};
use crate::frontend::sims_ims_frontend::{ItemInfo, Items, ShelfInfo, Shelves};
use crate::frontend::sims_ims_frontend::sims_frontend_client::SimsFrontendClient;
use crate::states::SimsClientState;
use crate::ui_messages::Message;
use crate::ui_messages::Message::{StartEditing, StopEditing, TabSelected, UpdatedItems, UpdatedShelves, UpdateItems, UpdateShelves};

mod assets;
mod frontend;
mod ui_messages;
mod states;
mod styles;
mod views;

const SERVER_ADDRESS: &str = "http://localhost:50051";

pub fn main() -> iced::Result {
    if cfg!(debug_assertions) {
        Builder::new()
            .filter_module("cs4471_sims_cli_client", LevelFilter::Debug)
            .init();
    }

    if !cfg!(macos) {
        set_var("WGPU_BACKEND", "vulkan");
    }
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
    shelves: Vec<ShelfInfo>,
    all_items: HashMap<String, Vec<ItemInfo>>
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
            shelves: Vec::new(),
            all_items: HashMap::new()
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
            Message::SlotPicked(s) => {
                match &mut self.edit_item {
                    None => {info!("Attempted to create shelf with no edit target"); Command::none()},
                    Some(target) => match target{
                        EditTarget::NewItem {shelf_id, ..} => {
                            *shelf_id = s;
                            Command::none()
                        },
                        _ => Command::none()
                    }
                }
            }
            Message::CreateTarget => {
                match &mut self.edit_item {
                    None => {info!("Attempted to create shelf with no edit target"); Command::none()},
                    Some(target) => match target{
                        EditTarget::NewShelf {shelf_name, slots, error_message} => {
                            if let Ok(count) = slots.parse::<u32>() {
                                if count <= 0 {
                                    error_message.insert("Your shelf must have at least 1 slot".to_owned());
                                    Command::none()
                                }
                                else {
                                    println!("Creating shelf with name {}", shelf_name);
                                    Command::batch([
                                        Command::perform(create_shelf(Arc::clone(&self.rpc), shelf_name.clone(), count, self.username.clone(), self.token.as_ref().unwrap().clone()), |result| {match result {Err(e)=>debug!("{:?}", e), _=>{}}; UpdateShelves(None) }),
                                        Command::perform(async{}, |_|{StopEditing})
                                    ])
                                }
                            }else{
                                error_message.insert("Slots must be a natural number".to_owned());
                                Command::none()
                            }
                        },
                        EditTarget::NewItem {item_name, item_count, shelf_id, error_message} => {
                            if let Ok(count) = item_count.parse::<u32>() {
                                if shelf_id == "" {
                                    error_message.insert("You must select a shelf".to_owned()); Command::none()
                                }else{
                                        let shelf = shelf_id.clone();
                                        Command::batch([
                                        Command::perform(
                                            create_item(
                                                Arc::clone(&self.rpc),
                                                shelf_id.clone(),
                                                count,
                                                self.username.clone(),
                                                self.token.as_ref().unwrap().clone(),
                                                item_name.clone()),
                                            |result| {match result {Err(e)=>debug!("{:?}", e), _=>{}}; UpdateItems(Some(shelf)) }),
                                        Command::perform(async{}, |_|{StopEditing})
                                    ])
                                }
                            }else{
                                error_message.insert("Slots must be a natural number".to_owned());
                                Command::none()
                            }
                        },
                        _ => Command::none()
                    }
                }
            }
            Message::ShelfSlotCountInputChanged(ref c) => {
                match &mut self.edit_item {
                    None => info!("Received {:?} when not editing anything", message),
                    Some(target)=> match target {
                        EditTarget::NewShelf { ref mut slots, .. } => {
                            *slots = c.clone()
                        },
                        EditTarget::NewItem {ref mut item_count, ..} => {
                            *item_count = c.clone()
                        }
                        _ => info!("Received message {:?} but current EditTarget is unsupported", message)
                    }
                };
                Command::none()
            }
            Message::CreateObjectNameInputChanged(ref s) => {
                match &mut self.edit_item {
                    None => info!("Received {:?} when not editing anything", message),
                    Some(target)=> match target {
                        EditTarget::NewShelf { ref mut shelf_name, .. } => {
                            *shelf_name = s.clone();
                        },
                        EditTarget::NewItem{ref mut item_name, .. } => {
                            *item_name = s.clone();
                        }
                        _ => info!("Received message {:?} but current EditTarget is unsupported", message)
                    }
                };
                Command::none()
            }
            Message::UpdateItems(shelf) => {
                Command::perform(read_items(Arc::clone(&self.rpc), shelf, self.username.clone(), self.token.as_ref().unwrap().clone()), Message::UpdatedItems)
            }
            Message::UpdatedItems(result) => match result {
                Ok(items) => {
                    debug!("{:?}", items);
                    match items {
                        GetItemsResponse::ShelfItems(shelf_id, items) => {
                            self.all_items.insert(shelf_id, items.items);
                        },
                        GetItemsResponse::AllItems(items) => {
                            self.all_items.clear();
                            self.all_items = items.items.into_iter().map(|i|(i.shelf_id.clone(), i)).fold(HashMap::new(), |mut h, v|{
                                match h.get_mut(&v.0) {
                                    Some(l)=> l.push(v.1),
                                    None => {h.insert(v.0, vec![v.1]);}
                                };
                                h
                            });
                        }
                    }
                    Command::none()
                },
                Err(e) => {
                    debug!("Items update failed: {:?}", e);
                     Command::none()
                }

            },
            Message::UpdateShelves(shelf_id) => {
                Command::perform(frontend::read_shelves(Arc::clone(&self.rpc), shelf_id, self.username.clone(), self.token.as_ref().unwrap().clone()), UpdatedShelves)
            }
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
            },
            Message::RegisterButtonClicked => {
                if let SimsClientState::Unauthenticated { ref password, .. } = self.state {
                    let client_ = Arc::clone(&self.rpc);
                    let ret = Command::perform(
                        frontend::register_and_login(
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
            },
            Message::Authenticated(authentication_result) => {
                match authentication_result {
                    Ok(response) => {
                        debug!("token received: {}", response.token);
                        self.token = Some(response.token);

                        if matches!(self.state, SimsClientState::Authenticating { .. }) {
                            self.state = SimsClientState::InventoryView;
                            Command::perform(frontend::read_shelves(Arc::clone(&self.rpc), None, self.username.clone(), self.token.as_ref().unwrap().clone()), UpdatedShelves)
                        }else {
                            Command::none()
                        }
                    }
                    Err(err) => {
                        debug!("Failed to log in {:?}", err);
                        self.username = String::new();
                        self.state = SimsClientState::Unauthenticated {
                            password: String::new(),
                            error_message: Some(match err {
                                LoginResult::ServerError(e) => format!("Placeholder: {:?}", e), //TODO: fill in text
                                LoginResult::NotConnected => {
                                    "Could not connect to server".to_owned()
                                },
                                LoginResult::RegisterFailed => {
                                    "Failed to register".to_owned()
                                }
                            }),
                        };
                        Command::none()
                    }
                }
            }
            Message::TabSelected(tab_id) => {
                debug!("Selected tab {:?}", tab_id);
                self.current_tab.push(tab_id.clone());
                match tab_id {
                    TabId::AllItems => {
                        Command::perform(read_items(Arc::clone(&self.rpc), None, self.username.clone(), self.token.as_ref().unwrap().clone()), UpdatedItems)
                    }
                    TabId::AllShelves => {
                        Command::perform(read_shelves(Arc::clone(&self.rpc), None, self.username.clone(), self.token.as_ref().unwrap().clone()), UpdatedShelves)
                    }
                    TabId::ShelfView(shelf_id) => {
                        Command::perform(read_items(Arc::clone(&self.rpc), Some(shelf_id), self.username.clone(), self.token.as_ref().unwrap().clone()), UpdatedItems)
                    }
                }
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
                }

                Command::perform(async {tab_id}, TabSelected)
            }
            StopEditing => {
                self.edit_item = None;
                Command::none()
            }
            StartEditing(target) => {
                self.edit_item = Some(target);
                Command::none()
            }
            UpdatedShelves(shelves) => {
                match shelves {
                    Ok(s) => {self.shelves = s.shelves;}
                    Err(e) => println!("{:?}", e)
                }
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
