use crate::{
    entities::{
        MatchStateStreamWrapper, 
        MatchStatus,
        MatchState,
        ActionMade,
        Player,
    },
    helpers::{
        get_player_from_token, 
        get_round_winner_id, 
        get_token_from_metadata,
        insert_player_to_map,
        get_enemy_player,
        match_state,
    },
    jajanken::{
        jajanken_server::Jajanken, 
        GetEnemyMovementResponse, 
        SelectMovementRequest
    },
    PlayerMap,
};
use serde_json;
use tonic::{Request, Response, Status};
#[derive(Debug, Default)]
pub struct MatchService {
    player_map: PlayerMap,
    match_stream: MatchStateStreamWrapper
}

impl MatchService {
    fn update_match_state(
        &self, 
        mut player: Player
    ) {
        let mut match_state_lock = self.match_stream.write().unwrap();

        let last_match_state = match_state_lock.last().unwrap().clone();
        let mut enemy_player = match_state::get_enemy(
            &last_match_state,
            player.id.to_owned()
        ).unwrap();
        
        let player_movement = player.current_movement.to_owned();
        let enemy_movement = enemy_player.current_movement.to_owned();
 
        let round_winner_id = get_round_winner_id(
            &player_movement.unwrap_or_default(),
            &enemy_movement.unwrap_or_default()
        );

        let get_new_score = |score: Option<u8>| {
            match score {
                Some(score) => {
                    return Some(score + 1);
                },
                None => {
                    return Some(1);
                }
            }
        };

        match round_winner_id {
            "player" => {
                player.score = get_new_score(player.score); 
            },
            "enemy" => {
                enemy_player.score = get_new_score(enemy_player.score);
            },
            "draw" => {},
            _ => {
                match_state_lock.push(
                    MatchState {
                        rounds_played: last_match_state.rounds_played,
                        action_made: ActionMade::MadeMovement,
                        actioned_by: Some(player.display_name.to_owned()),
                        status: last_match_state.status.clone(),
                        player_list: vec![player, enemy_player],
                    }
                );
                return;
            }
        }

        let player_list = vec![
            player.clone(), 
            enemy_player.clone()
        ];

        let new_status = match player_list.clone().into_iter().any(
            |current_player| current_player.score.unwrap() == 3
        ) {
            true => MatchStatus::Finished,
            false => MatchStatus::InProgress,
        };

        match_state_lock.push(
            MatchState {
                rounds_played: last_match_state.rounds_played + 1,
                action_made: ActionMade::RoundFinished,
                actioned_by: Some(player.display_name.to_owned()),
                status: new_status,
                player_list,
            }
        );
    }

    fn on_player_join (&self, token: &str, new_player: Player) {
        let player_map_lock = self.player_map.read().unwrap();
        let mut match_state_lock = self.match_stream.write().unwrap();
        if player_map_lock.len() != 2 || match_state_lock.len() != 0 {
            return;
        }
        let enemy_player = get_enemy_player(&self.player_map, token).unwrap();
        match_state_lock.push(
            MatchState {
                status: MatchStatus::InProgress,
                player_list: vec![
                    new_player.clone(),
                    enemy_player.clone()
                ],
                action_made: ActionMade::StartedGame,
                actioned_by: None,
                rounds_played: 0
            }
        );
    }
}

#[tonic::async_trait]
impl Jajanken for MatchService {
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
                insert_player_to_map(&self.player_map, new_player.clone(), token);

                self.on_player_join(token, new_player);
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

    async fn select_movement(
        &self,
        request: Request<SelectMovementRequest>,
    ) -> Result<Response<()>, Status> {
        let match_state_lock = self.match_stream.read().unwrap();
        
        if match_state_lock.len() == 0 {
            return Err(Status::new(
                tonic::Code::Unavailable,
                "Match not started",
            ));
        }

        let metadata = request.metadata().clone();
        let token = get_token_from_metadata(&metadata);
        let player_from_token = get_player_from_token(&self.player_map, token);

        if player_from_token.is_none() {
            return Err(Status::new(
                tonic::Code::Unauthenticated,
                "player not found",
            ));
        }

        let last_match_state = match_state_lock.last().unwrap();

        let player_match_state = match_state::get_player(
            last_match_state,
            player_from_token.unwrap().id.to_owned()
        );

        let mut player = player_match_state.unwrap();
        let movement_request = request.into_inner();

        player.current_movement = Some(movement_request.movement);
        
        self.update_match_state(player);

        Ok(Response::new(()))
    }

    async fn get_enemy_movement(
        &self,
        _request: Request<()>,
    ) -> Result<Response<GetEnemyMovementResponse>, Status> {
        Ok(Response::new(GetEnemyMovementResponse {
            enemy_movement: "paper".to_string(),
        }))
    }

    async fn get_match_state(&self, _request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::new(
            tonic::Code::Unimplemented,
            "Method not implemented",
        ))
    }
}
