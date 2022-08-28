use entities::Player;
use jajanken::jajanken_server::JajankenServer;
use services::MatchService;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tonic::transport::Server;

mod entities;
mod helpers;
mod services;

pub mod jajanken {
    tonic::include_proto!("jajanken");
}

type PlayerMap = Arc<RwLock<HashMap<String, Player>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let match_service = JajankenServer::new(MatchService::default());

    Server::builder()
        .add_service(match_service)
        .serve(addr)
        .await?;

    Ok(())
}
