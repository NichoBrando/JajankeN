use crate::entities::{Player, MatchState};

pub fn get_enemy (match_state: &MatchState, current_player_id: String) -> Option<Player> {
    match match_state.player_list.iter().find(|player| player.id != current_player_id) {
        Some(player) => Some(player.clone()),
        None => None,
    }
}

pub fn get_player (match_state: &MatchState, current_player_id: String) -> Option<Player> {
    match match_state.player_list.iter().find(|player| player.id == current_player_id) {
        Some(player) => Some(player.clone()),
        None => None,
    }
}