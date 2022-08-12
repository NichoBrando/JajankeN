
use crate::{
    PlayerMap,
    jajanken::{
        jajanken_server::Jajanken,
        GetEnemyMovementResponse, 
        SelectMovementRequest
    },
    helpers::{
        insert_player_to_map,
        get_player_from_token,
        get_token_from_metadata
    },
    entities::{
        Player
    }
};
use tonic::{ Request, Response, Status };
use serde_json;

#[derive(Debug, Default)]
pub struct MatchService {
    player_map: PlayerMap,
}

#[tonic::async_trait]
impl Jajanken for MatchService {
    async fn select_movement(
        &self,
        request: Request<SelectMovementRequest>,
    ) -> Result<Response<()>, Status> {
        let token = get_token_from_metadata(&request.metadata());

        match get_player_from_token(&self.player_map, token) {
            Some(_player) => {
                return Ok(Response::new(()));
            }
            None => {
                println!("player not found");
                return Err(Status::new(
                    tonic::Code::Unauthenticated,
                    "player not found",
                ));
            }
        }
    }

    async fn get_enemy_movement(
        &self,
        _request: Request<()>,
    ) -> Result<Response<GetEnemyMovementResponse>, Status> {
        Ok(Response::new(GetEnemyMovementResponse {
            enemy_movement: "paper".to_string(),
        }))
    }

    async fn login(&self, _request: Request<()>) -> Result<Response<()>, Status> {
        let token = get_token_from_metadata(&_request.metadata());

        let url = "http://localhost:8000/user".to_string();
        let client = reqwest::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(token).unwrap(),
        );

        match client.get(&url).headers(headers).send().await {
            Ok(response) => {
                let body = response.text().await.unwrap();
                let new_player: Player = serde_json::from_str(&body).unwrap();
                insert_player_to_map(&self.player_map, new_player, token);
                return Ok(Response::new(()));
            }
            Err(e) => {
                println!("{:?}", e);
                Err(Status::new(
                    tonic::Code::Unauthenticated,
                    "player not found",
                ))
            }
        }
    }
}