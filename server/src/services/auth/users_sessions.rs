use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::services::auth::jwt;

pub type UsersSessions = Arc<RwLock<HashMap<String, String>>>;

pub async fn initialize_users_sessions() -> UsersSessions {
    Arc::new(RwLock::new(HashMap::new()))
}

pub async fn get_logged_user_id(user_sessions_lock: &UsersSessions, token: &String) -> Result<String, bool> {
    match user_sessions_lock.read() {
        Ok(user_sessions) => {
            let user_id = jwt::get_user_id_from_jwt(&token);

            match user_sessions.get(&user_id) {
                Some(saved_token) => {
                    match saved_token == token {
                        true => Ok(user_id),
                        false => Err(false),
                    }
                },
                None => Err(false),
            }

        },
        Err(_) => Err(false),
    }
}

pub async fn update_logged_user(user_sessions_lock: &UsersSessions, user_id: &String, token: &String) {
    match user_sessions_lock.write() {
        Ok(mut user_sessions) => {
            user_sessions.insert(user_id.clone(), token.clone());
        },
        Err(_) => {},
    }
}