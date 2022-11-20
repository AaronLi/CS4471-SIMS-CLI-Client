use async_std::sync::Arc;
use env_logger::Builder;
use iced::{Application, Command, Element, executor, Length, Settings, Theme};
use iced::futures::lock::Mutex;
use iced::futures::TryFutureExt;
use iced::Length::Fill;
use iced::widget::{button, column, container, row, Space, text, TextInput};
use log::{debug, error, info, LevelFilter};
use tonic::transport::{Channel};

use crate::frontend::sims_ims_frontend::sims_frontend_client::SimsFrontendClient;
use crate::iced_messages::Message;
use crate::states::SimsClientState;

mod states;
mod frontend;
mod iced_messages;


pub fn main() -> iced::Result {
    Builder::new()
        .filter_module("cs4471_sims_cli_client", LevelFilter::Debug)
        .init();
    ClientState::run(Settings::default())
}


#[derive(Debug)]
struct ClientState {
    username: String,
    state: SimsClientState,
    rpc: Arc<Mutex<Option<SimsFrontendClient<Channel>>>>,
    token: Option<String>,
    previous_view: Option<SimsClientState>
}

impl Application for ClientState {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>)  {
        let new_client = ClientState {
            username: String::new(),
            state: SimsClientState::Unauthenticated{
                password: "".to_string()
            },
            rpc: Arc::new(Mutex::new(None)),
            token: None,
            previous_view: None
        };
        let rpc_ = Arc::clone(&new_client.rpc);
        (
            new_client,
            Command::perform(frontend::connect("http://localhost:50051".to_owned(), rpc_).map_err(|e|{format!("{:?}", e)}), Message::Connected)
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
                if let SimsClientState::Unauthenticated { ref password } = self.state {
                    let client_ = Arc::clone(&self.rpc);
                    let ret = Command::perform(frontend::login(client_, self.username.to_owned(), password.to_owned()), Message::Authenticated);
                    self.state = SimsClientState::Authenticating;
                    ret
                }else {
                    // login button clicked in non-login screen
                    Command::none()
                }
            }

            Message::Connected(connect_result) => {
                match connect_result {
                    Ok(_) => {info!("Connected")}
                    Err(e) => {
                        error!("Failed to connect: {}", e)
                    }
                }
                Command::none()
            }
            Message::Authenticated(authentication_result) => {
                match authentication_result {
                    Ok(response) => {
                        debug!("token received: {}", response.token);
                        self.token = Some(response.token);

                        if matches!(self.state, SimsClientState::Authenticating{..}) {
                            self.state = SimsClientState::AutomaticViewSelection;
                            Command::perform(async {}, Message::SelectScene)
                        }else {
                            Command::none()
                        }
                    }
                    Err(err) => {
                        info!("Failed to log in");
                        self.username = String::new();
                        self.state = SimsClientState::Unauthenticated {password: String::new()};
                        Command::none()
                    }
                }
            },
            Message::SelectScene(_) => {
                match &self.previous_view {
                    None => self.state = SimsClientState::ShelfView,
                    Some(state) => match state {
                        SimsClientState::ShelfItemView => {self.state = SimsClientState::ShelfItemView}
                        SimsClientState::ShelfView => {self.state = SimsClientState::ShelfView}
                        _ => {self.state = SimsClientState::ShelfView}

                    }
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match &self.state {
            SimsClientState::Unauthenticated { password} => {
                let elements = row![Space::with_width(Length::FillPortion(1)), column![
                    container(text("SIMS IMS")).width(Fill).center_x(),
                    TextInput::new("Username", &self.username, Message::UsernameInputChanged),
                    TextInput::new("Password", password, Message::PasswordInputChanged),
                    container(button("Login").on_press(Message::LoginButtonClicked)).width(Fill).center_x()
                ].width(Length::FillPortion(1)), Space::with_width(Length::FillPortion(1))];

                container(elements)
                    .width(Fill)
                    .height(Fill)
                    .center_x()
                    .center_y()
                    .into()
            },
            SimsClientState::Authenticating {..} => {
                container("Logging In...")
                    .width(Fill)
                    .height(Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
            SimsClientState::AutomaticViewSelection => {
                container("Logged in!")
                    .width(Fill)
                    .height(Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
            _ => container(text(format!("Placeholder for state: {:?}", self.state))).width(Fill).height(Fill).center_x().center_y().into()
        }
    }
}