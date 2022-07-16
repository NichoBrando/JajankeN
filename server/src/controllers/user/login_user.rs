use crate::models::user::{User};
use crate::services::auth::{users_sessions, jwt};
use crate::services::{database, encryption};
use mongodb::bson::{doc};
use mongodb::Collection;
use rocket::State;
use rocket::http::Status;
use serde_json;
use std::convert::From;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct LoginInput {
    username: String,
    password: String
}

#[post("/login", data = "<login_input>", format = "json")]
pub async fn login(login_input: String, users_sessions_lock: &State<users_sessions::UsersSessions>) -> Result<String, Status> {
    let login_data: LoginInput = serde_json::from_str(&login_input.to_string()).unwrap();
    let username = login_data.username.trim().to_string();
    let password = login_data.password.trim().to_string();
    let db_session = database::connect().await.unwrap();
    let users_collection: Collection<User> = database::get_collection(&db_session, "users");
    let user_query = doc! {
        "username": username
    };
    let user = users_collection
        .find_one(user_query, None)
        .await
        .unwrap();
    match user {
        Some(user) => {
            match user.password {
                Some(user_password) => {
                    let is_authenticated = encryption::verify_password(&password, &user_password);
                    if !is_authenticated {
                        Err(Status::Unauthorized)
                    }
                    else {
                        match user.id {
                            Some(user_id) => {
                                let user_id_string: String = database::get_object_id_string(&user_id);
                                let token = jwt::create_jwt(&user_id_string);
                                users_sessions::update_logged_user(&users_sessions_lock, &user_id_string, &token).await;
                                Ok(token)
                            },
                            None => Err(Status::Unauthorized)
                        }
                    }
                },
                None => {
                    Err(Status::Unauthorized)
                }
            }
        }
        None => {
            Err(Status::Unauthorized)
        }
    }
}
