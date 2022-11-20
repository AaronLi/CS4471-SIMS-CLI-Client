use async_std::sync::Arc;
use iced::futures::lock::{Mutex};
use tonic::transport::{Channel, Error};
use sims_ims_frontend::sims_frontend_client::SimsFrontendClient;
use crate::frontend::LoginResult::{NotConnected, ServerError};
use crate::frontend::sims_ims_frontend::{LoginRequest, Token};

pub mod sims_ims_frontend {
    tonic::include_proto!("sims_ims_frontend");
}

pub(crate) async fn connect(address: String,rpc: Arc<Mutex<Option<SimsFrontendClient<Channel>>>>) -> Result<(), Error> {
    match SimsFrontendClient::connect(address).await {
        Ok(client) => {
            let _ = rpc.lock().await.insert(client);
            Ok(())
        }
        Err(e) => {
            Err(e)
        }
    }

}

#[derive(Debug, Clone)]
pub(crate) enum LoginResult {
    ServerError(tonic::Status),
    NotConnected
}

pub(crate) async fn login(rpc: Arc<Mutex<Option<SimsFrontendClient<Channel>>>>, username: String, password: String) -> Result<Token, LoginResult> {
    match rpc.lock().await.as_mut() {
        None => {Err(NotConnected)}
        Some(client_rpc) => {
            client_rpc.cred_auth(LoginRequest{
                username,
                password
            }).await.map(|r|r.into_inner()).map_err(|e| ServerError(e))
        }
    }
}