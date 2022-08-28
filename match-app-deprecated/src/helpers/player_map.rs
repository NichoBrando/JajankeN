use crate::entities::Player;
use crate::PlayerMap;

pub fn get_player_from_token(player_map: &PlayerMap, token: &str) -> Option<Player> {
    match player_map.read() {
        Ok(readable_player_map) => match readable_player_map.get(token) {
            Some(player) => Some(player.clone()),
            None => None,
        },
        Err(_) => None,
    }
}

pub fn insert_player_to_map(player_map: &PlayerMap, player: Player, token: &str) {
    let mut player_map = player_map.write().unwrap();
    player_map.insert(token.to_owned(), player);
}

pub fn get_enemy_player(player_map: &PlayerMap, current_player_token: &str) -> Option<Player> {
    match player_map.read() {
        Ok(readable_player_map) => {
            for (player_token, player) in readable_player_map.iter() {
                if player_token != current_player_token {
                    return Some(player.to_owned());
                }
            }
            None
        }
        Err(_) => None,
    }
}