use crate::frontend::sims_ims_frontend::{LoginRequest, Token};
use crate::frontend::LoginResult::{NotConnected, ServerError};
use async_std::sync::Arc;
use iced::futures::lock::Mutex;
use iced::widget::{container, row, Button, Svg, Row, Space, text, button, svg};
use iced::Length;
use iced::Length::{Fill, Shrink};
use iced::widget::image::Handle;
use sims_ims_frontend::sims_frontend_client::SimsFrontendClient;
use tonic::transport::{Channel, Error};
use crate::assets::CLOSE_ICON_SVG;
use crate::frontend::TabId::AllShelves;
use crate::iced_messages::Message;
use crate::iced_messages::Message::{CloseShelf, TabSelected};

pub mod sims_ims_frontend {
    tonic::include_proto!("sims_ims_frontend");
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub(crate) enum TabId {
    AllShelves,
    AllItems,
    ShelfItems(String)
}

impl Default for &TabId {
    fn default() -> Self {
        &AllShelves
    }
}

#[derive(Debug, Clone)]
pub(crate) enum LoginResult {
    ServerError(tonic::Status),
    NotConnected,
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

pub(crate) fn create_tab<'a>(tab_id: TabId, text_content: String, closeable: bool, icon: Option<Svg>) -> Button<'a, Message> {
    let mut button_display = Row::new();

    if let Some(svg) = icon {
        button_display = button_display
            .push(svg.width(Shrink))
            .push(Space::with_width(Length::Units(5)));
    }

    button_display = button_display.push(text(text_content));

    if closeable {
        button_display = button_display.push(Space::with_width(Length::Units(5))).push(
            Button::new(Svg::new(svg::Handle::from_memory(CLOSE_ICON_SVG)).width(Shrink)).on_press(CloseShelf(tab_id.clone()))
        )
    }

    Button::new(
        container(button_display)
        .width(Shrink)
        .height(Fill)
        .center_y(),
    )
    .width(Shrink)
    .height(Length::Units(30))
        .on_press(TabSelected(tab_id))
}
