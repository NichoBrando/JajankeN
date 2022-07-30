use tonic::{transport::Server, Request, Response, Status};

use jajanken::jajanken_server::{Jajanken, JajankenServer};
use jajanken::{SelectMovementRequest, GetEnemyMovementResponse};

pub mod jajanken {
    tonic::include_proto!("jajanken");
}

#[derive(Debug, Default)]
pub struct MatchServer {}

#[tonic::async_trait]
impl Jajanken for MatchServer {
    async fn select_movement(
        &self,
        _request: Request<SelectMovementRequest>,
    ) -> Result<Response<()>, Status> {
        println!("Got a request: {:?}", _request);
        Ok(Response::new(()))
    }

    async fn get_enemy_movement(
        &self,
        _request: Request<()>
    ) -> Result<Response<GetEnemyMovementResponse>, Status> {
        Ok(
            Response::new(
                GetEnemyMovementResponse {
                    enemy_movement: "paper".to_string()
                }
            )
        )
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let match_server = MatchServer::default();

    Server::builder()
        .add_service(JajankenServer::new(match_server))
        .serve(addr)
        .await?;

    Ok(())
}