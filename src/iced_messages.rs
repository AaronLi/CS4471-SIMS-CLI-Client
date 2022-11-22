use crate::frontend::{EditTarget, LoginResult, TabId};
use crate::frontend::sims_ims_frontend::Token;

#[derive(Debug, Clone)]
pub(crate) enum Message {
    LoginButtonClicked,
    UsernameInputChanged(String),
    PasswordInputChanged(String),
    Authenticated(Result<Token, LoginResult>),
    TabSelected(TabId),
    CloseShelf(TabId),
    OpenShelf(TabId),
    StartEditing(EditTarget),
    StopEditing
}