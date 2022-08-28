use std::sync::{Arc, RwLock};

use crate::entities::Player;

#[derive(Clone, Debug)]
pub enum MatchStatus {
    Pending,
    InProgress,
    Finished,
}

impl Default for MatchStatus {
    fn default() -> Self {
        MatchStatus::Pending
    }
}

#[derive(Clone, Debug)]
pub enum ActionMade {
    StartedGame,
    MadeMovement,
    RoundFinished,
    NoAction
}

impl Default for ActionMade {
    fn default() -> Self {
        ActionMade::NoAction
    }
}

#[derive(Clone, Default, Debug)]
pub struct MatchState {
    pub rounds_played: u32,
    pub action_made: ActionMade,
    pub actioned_by: Option<String>,
    pub status: MatchStatus,
    pub player_list: Vec<Player>,
}

pub type MatchStateStreamWrapper = Arc<RwLock<Vec<MatchState>>>;
