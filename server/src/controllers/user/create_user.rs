use crate::models::user::{User, UserInfo};
use crate::services::{database, encryption};
use mongodb::bson::{doc, Regex};
use mongodb::options::FindOneOptions;
use mongodb::Collection;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json;
use std::convert::From;

#[post("/", data = "<user_input>", format = "json")]
pub async fn create(user_input: String) -> Result<Json<UserInfo>, Status> {
    let mut user: User = serde_json::from_str(&user_input).unwrap();
    user.display_name = user.display_name.trim().to_string();
    user.username = user.username.trim().to_string();

    let db_session = database::connect().await.unwrap();
    let users_collection: Collection<User> = database::get_collection(&db_session, "users");

    let display_name_regex = Regex {
        pattern: format!("^{}$", user.display_name),
        options: "i".to_string(),
    };

    let existing_display_name_query = doc! {
        "display_name": display_name_regex
    };

    let is_display_name_in_use = users_collection
        .count_documents(existing_display_name_query, None)
        .await
        .unwrap();

    if is_display_name_in_use > 0 {
        return Err(Status::Conflict);
    }

    let username_regex = Regex {
        pattern: format!("^{}$", user.username),
        options: "i".to_string(),
    };

    let existing_username_query = doc! {
        "username": username_regex
    };

    let is_username_in_use = users_collection
        .count_documents(existing_username_query, None)
        .await
        .unwrap();

    if is_username_in_use > 0 {
        return Err(Status::Conflict);
    }

    match user.password {
        Some(mut password) => {
            password = password.trim().to_string();
            if password.len() < 5 {
                return Err(Status::BadRequest);
            }
            user.password = Some(encryption::hash_password(&password));
        }
        None => {
            return Err(Status::BadRequest);
        }
    }

    if user.display_name.len() < 3 {
        return Err(Status::BadRequest);
    }

    if user.username.len() < 6 {
        return Err(Status::BadRequest);
    }

    user.id = Some(database::get_new_object_id());

    match users_collection.insert_one(user, None).await {
        Ok(result) => {
            let user_id = result.inserted_id;
            let user_project = doc! {
                "_id": 0,
                "display_name": 1,
                "image_url": 1
            };
            let find_one_options = Some(FindOneOptions::builder().projection(user_project).build());
            let result_users_collection: Collection<UserInfo> =
                database::get_collection(&db_session, "users");
            let created_user: Option<UserInfo> = result_users_collection
                .find_one(Some(doc! { "_id": user_id }), find_one_options)
                .await
                .unwrap();
            match created_user {
                Some(user) => Ok(Json(user)),
                None => {
                    return Err(Status::InternalServerError);
                }
            }
        }
        Err(_) => {
            return Err(Status::InternalServerError);
        }
    }
}
