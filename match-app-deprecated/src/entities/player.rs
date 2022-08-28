use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Player {
    pub id: String,
    pub display_name: String,
    pub score: Option<u8>,
    pub current_movement: Option<String>,
}