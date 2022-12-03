use std::sync::mpsc;
use crate::frontend::{EditTarget, GetItemsResponse, LoginResult, RpcCallResult, TabId};
use crate::frontend::sims_ims_frontend::{ShelfInfo, Shelves, Token};

#[derive(Debug, Clone)]
pub(crate) enum Message {
    LoginButtonClicked,
    RegisterButtonClicked,
    UsernameInputChanged(String),
    PasswordInputChanged(String),
    Authenticated(Result<Token, LoginResult>),
    TabSelected(TabId),
    CloseShelf(TabId),
    OpenShelf(TabId),
    StartEditing(EditTarget),
    StopEditing,
    UpdatedShelves(Result<Shelves, RpcCallResult>),
    UpdateShelves,
    UpdatedItems(Result<GetItemsResponse, RpcCallResult>)
}