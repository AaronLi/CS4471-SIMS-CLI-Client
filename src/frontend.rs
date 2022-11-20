use async_std::sync::Arc;
use iced::futures::lock::{Mutex};
use tonic::transport::{Channel, Error};
use sims_ims_frontend::sims_frontend_client::SimsFrontendClient;
use crate::frontend::LoginResult::{NotConnected, ServerError};
use crate::frontend::sims_ims_frontend::{LoginRequest, Token};

pub mod sims_ims_frontend {
    tonic::include_proto!("sims_ims_frontend");
}

#[derive(Debug, Clone)]
pub(crate) enum LoginResult {
    ServerError(tonic::Status),
    NotConnected
}

pub(crate) async fn login(rpc: Arc<Mutex<Option<SimsFrontendClient<Channel>>>>, address: String, username: String, password: String) -> Result<Token, LoginResult> {
    let mut rpc_present = match rpc.lock().await.take() {
        None => {SimsFrontendClient::connect(address).await.map_err(|_|NotConnected)?}
        Some(client_rpc) => {
            client_rpc
        }
    };

    let response = rpc_present.cred_auth(LoginRequest{username, password}).await.map(|x|x.into_inner()).map_err(|e|LoginResult::ServerError(e));
    let _ = rpc.lock().await.insert(rpc_present);

    response
}