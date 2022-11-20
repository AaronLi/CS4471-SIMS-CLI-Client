use crate::frontend::LoginResult;
use crate::frontend::sims_ims_frontend::Token;

#[derive(Debug, Clone)]
pub(crate) enum Message {
    LoginButtonClicked,
    UsernameInputChanged(String),
    PasswordInputChanged(String),
    Connected(Result<(), String>),
    Authenticated(Result<Token, LoginResult>),
    SelectScene(())
}