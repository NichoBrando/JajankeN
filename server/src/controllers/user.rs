use crate::models::user::User;
use crate::services::{database, encryption};
use mongodb::bson::{doc, Regex};
use mongodb::Collection;
use serde_json;

#[derive(FromForm)]
pub struct UserInput {
    pub display_name: String,
    pub username: String,
    pub password: String,
}

#[get("/")]
pub fn get() -> String {
    // TODO: ADD REAL FUNCTIONALITY TO GET USER
    let user = User {
        display_name: "John Doe".to_string(),
        image_url: Some("https://i.pravatar.cc/300".to_string()),
        username: "adacom".to_string(),
        password: None,
        id: Some(database::get_new_object_id()),
    };
    let result = serde_json::to_string(&user).unwrap();
    result
}

#[post("/", data = "<user_input>")]
pub async fn create(user_input: String) -> String {
    // TODO: ADD MORE EXCEPTION HANDLING
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
        return "Display name already in use".to_string();
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
        return "Invalid username".to_string();
    }

    match user.password {
        Some(mut password) => {
            password = password.trim().to_string();
            if password.len() < 5 {
                return "Invalid password".to_string();
            }
            user.password = Some(encryption::hash_password(&password));
        }
        None => {
            return "Missing password".to_string();
        }
    }

    if user.display_name.len() < 3 {
        return "".to_string();
    }

    if user.username.len() < 6 {
        return "".to_string();
    }

    user.id = Some(database::get_new_object_id());

    match users_collection.insert_one(user, None).await {
        Ok(result) => {
            let user_id = result.inserted_id;
            let created_user = users_collection
                .find_one(Some(doc! { "_id": user_id }), None)
                .await
                .unwrap();
            match created_user {
                Some(mut user_response) => {
                    user_response.password = None;
                    serde_json::to_string(&user_response).unwrap()
                }
                None => "".to_string(),
            }
        }
        Err(_) => "".to_string(),
    }
}
