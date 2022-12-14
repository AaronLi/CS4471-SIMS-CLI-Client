use crate::frontend::sims_ims_frontend::{ActionApproved, CreateShelfRequest, GetItemRequest, GetItemsRequest, GetShelvesRequest, Items, LoginRequest, ShelfInfo, Shelves, Token};
use crate::frontend::LoginResult::{NotConnected, RegisterFailed, ServerError};
use async_std::sync::Arc;
use iced::futures::lock::Mutex;
use iced::widget::{Button, Container, Row, Space, Svg, svg, Text};
use iced::{Length, Subscription, subscription};
use iced::futures::{StreamExt, TryStreamExt};
use iced::Length::{Fill, Shrink};
use tonic::codegen::Body;
use sims_ims_frontend::sims_frontend_client::SimsFrontendClient;
use tonic::transport::Channel;
use crate::assets::{CLOSE_ICON, get_icon};
use crate::frontend::TabId::AllShelves;
use crate::ui_messages;
use crate::ui_messages::Message;
use crate::ui_messages::Message::{CloseShelf, TabSelected};

pub mod sims_ims_frontend {
    tonic::include_proto!("sims_ims_frontend");
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub(crate) enum TabId {
    AllShelves,
    AllItems,
    ShelfView(String)
}

impl Default for &TabId {
    fn default() -> Self {
        &AllShelves
    }
}

#[derive(Debug, Clone)]
pub(crate) enum EditTarget {
    EditShelf{shelf_id: String},
    EditItem{shelf_id: String, item_id: String},
    EditSlot{shelf_id: String, slot_id: String},
    NewItem{shelf_id: String, item_name: String, item_count: String, error_message: Option<String>},
    NewShelf{shelf_name: String, slots: String, error_message: Option<String>}
}

#[derive(Debug, Clone)]
pub(crate) enum LoginResult {
    ServerError(tonic::Status),
    NotConnected,
    RegisterFailed
}

#[derive(Debug, Clone)]
pub enum RpcCallResult {
    NotConnected,
    CallFailed(String)
}

#[derive(Debug, Clone)]
pub enum GetItemsResponse {
    AllItems(Items),
    ShelfItems(String, Items)
}

pub(crate) async fn login(
    rpc: Arc<Mutex<Option<SimsFrontendClient<Channel>>>>,
    address: String,
    username: String,
    password: String,
) -> Result<Token, LoginResult> {
    let mut rpc_present = match rpc.lock().await.take() {
        None => SimsFrontendClient::connect(address)
            .await
            .map_err(|_| NotConnected)?,
        Some(client_rpc) => client_rpc,
    };

    let response = rpc_present
        .cred_auth(LoginRequest { username, password })
        .await
        .map(|x| x.into_inner())
        .map_err(|e| ServerError(e));
    let _ = rpc.lock().await.insert(rpc_present);

    response
}

pub(crate) async fn register_and_login(
    rpc: Arc<Mutex<Option<SimsFrontendClient<Channel>>>>,
    address: String,
    username: String,
    password: String,
) -> Result<Token, LoginResult> {
    let mut rpc_present = match rpc.lock().await.take() {
        None => SimsFrontendClient::connect(address)
            .await
            .map_err(|_| NotConnected)?,
        Some(client_rpc) => client_rpc,
    };

    let _register_response = rpc_present.sign_up(LoginRequest{
        username: username.clone(),
        password: password.clone(),
    }).await.map_err(|_|RegisterFailed)?;

    let response = rpc_present
        .cred_auth(LoginRequest { username, password })
        .await
        .map(|x| x.into_inner())
        .map_err(|e| ServerError(e));
    let _ = rpc.lock().await.insert(rpc_present);

    response
}


pub(crate) fn create_tab<'a>(tab_id: TabId, text_content: String, closeable: bool, icon: Option<char>) -> Button<'a, Message> {
    let mut button_display = Row::new();

    if let Some(c) = icon {
        let icon_image = get_icon(c);
        button_display = button_display
            .push(icon_image)
            .push(Space::with_width(Length::Units(5)));
    }

    button_display = button_display.push(Text::new(text_content));

    if closeable {
        button_display = button_display.push(Space::with_width(Length::Units(5))).push(
            Button::new(Svg::new(svg::Handle::from_memory(CLOSE_ICON)).width(Length::Shrink)).on_press(CloseShelf(tab_id.clone()))
        )
    }

    Button::new(
        Container::new(button_display)
        .width(Shrink)
        .height(Fill)
        .center_y(),
    )
    .width(Shrink)
    .height(Length::Units(30))
        .on_press(TabSelected(tab_id))
}


pub(crate) async fn read_shelves(rpc: Arc<Mutex<Option<SimsFrontendClient<Channel>>>>, shelf_id: Option<String>, username: String, token: String) -> Result<Shelves, RpcCallResult> {
    match rpc.lock().await.as_mut() {
        None => return Err(RpcCallResult::NotConnected),
        Some(client_rpc) => client_rpc.get_shelves(GetShelvesRequest{
            shelf_id,
            username: username.to_owned(),
            token: token.to_owned()})
            .await
            .map_err(|e|RpcCallResult::CallFailed(e.to_string())).map(|r|r.into_inner()),
    }
}

pub(crate) async fn read_items(rpc: Arc<Mutex<Option<SimsFrontendClient<Channel>>>>, shelf_id: Option<String>, username: String, token: String) -> Result<GetItemsResponse, RpcCallResult> {
    match rpc.lock().await.as_mut() {
        None => return Err(RpcCallResult::NotConnected),
        Some(client_rpc) => client_rpc.get_items(GetItemsRequest{
            shelf_id: shelf_id.clone(),
            username: username.to_owned(),
            token: token.to_owned()})
            .await
            .map_err(|e|RpcCallResult::CallFailed(e.to_string())).map(|r|
            match shelf_id {
                Some(shelf)=> GetItemsResponse::ShelfItems(shelf, r.into_inner()),
                None => GetItemsResponse::AllItems(r.into_inner())
            }),
    }
}

pub(crate) async fn create_shelf(rpc: Arc<Mutex<Option<SimsFrontendClient<Channel>>>>, shelf_id: String, num_slots: u32, username: String, token: String) -> Result<ActionApproved, RpcCallResult> {
    match rpc.lock().await.as_mut() {
        None => return Err(RpcCallResult::NotConnected),
        Some(client_rpc) => client_rpc.create_shelf(CreateShelfRequest{
            username,
            token,
            shelfinfo: Some(ShelfInfo{ shelf_id, shelf_count: num_slots }),
        }).await.map_err(|e|RpcCallResult::CallFailed(e.to_string())).map(|r|r.into_inner())
    }
}

pub(crate) async fn create_item(rpc: Arc<Mutex<Option<SimsFrontendClient<Channel>>>>, shelf_id: String, count: u32, username: String, token: String, item_name: String) -> Result<(), RpcCallResult> {
    // match rpc.lock().await.as_mut() {
    //     None => Err(RpcCallResult::NotConnected),
    //     Some(client_rpc) => client_rpc.crea
    // }
    unimplemented!()
}