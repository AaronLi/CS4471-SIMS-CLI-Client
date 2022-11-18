use iced::{Element, Length, Sandbox, Settings};
use iced::Length::Fill;
use iced::widget::{container, column, button, text, scrollable, TextInput};
use crate::states::SimsClientState;

mod states;

#[derive(Debug, Clone)]
enum Message {
    LoginButtonClicked,
    UsernameInputChanged(String),
    PasswordInputChanged(String)
}

struct ClientState {
    state: SimsClientState
}

impl Sandbox for ClientState {
    type Message = Message;

    fn new() -> Self {
        ClientState {
            state: SimsClientState::Unauthenticated{
                username: "".to_string(),
                password: "".to_string()
            }
        }
    }

    fn title(&self) -> String {
        "SIMS Inventory Management System".to_owned()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::UsernameInputChanged(s) => {
                if let SimsClientState::Unauthenticated{ref mut username, .. } = self.state {
                    username.clear();
                    username.push_str(&s)
                }
            },
            Message::PasswordInputChanged(s) => {
                if let SimsClientState::Unauthenticated{ref mut password, .. } = self.state {
                    password.clear();
                    password.push_str(&s)
                }
            },
            Message::LoginButtonClicked => {
                if let SimsClientState::Unauthenticated { ref username, ref password} = self.state {
                    self.state = SimsClientState::Authenticating {
                        username: username.to_string(),
                        password: password.to_string()
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match &self.state {
            SimsClientState::Unauthenticated {username, password} => {
                let elements = column![
                    text("Hello, world!"),
                    TextInput::new("Username", username, Message::UsernameInputChanged),
                    TextInput::new("Password", password, Message::PasswordInputChanged),
                    button("Login").on_press(Message::LoginButtonClicked)
                ].width(Length::Units(200));

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
            _ => container("unimplemented").width(Fill).height(Fill).center_x().center_y().into()
        }
    }
}

fn main() -> iced::Result {
    ClientState::run(Settings::default())
}
