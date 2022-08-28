
pub fn get_round_winner_id<'a>(player_movement: &str, enemy_movement: &str) -> &'a str {
    if vec![player_movement, enemy_movement].contains(&"") {
        return "";
    }

    if player_movement == enemy_movement {
        return "draw";
    }
    
    match player_movement {
        "rock" => {
            if enemy_movement == "paper" {
                return "enemy";
            } else {
                return "player";
            }
        }
        "paper" => {
            if enemy_movement == "scissor" {
                return "enemy";
            } else {
                return "player";
            }
        }
        "scissor" => {
            if enemy_movement == "rock" {
                return "enemy";
            } else {
                return "player";
            }
        }
        _ => {
            return "draw";
        }
    }
}
