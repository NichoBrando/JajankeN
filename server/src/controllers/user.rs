use crate::models::user::User;
use crate::services::{database, encryption};
use mongodb::bson::doc;
use mongodb::Collection;
use serde_json;

#[derive(FromForm)]
pub struct UserInput {
    pub display_name: String,
    pub username: String,
    pub password: String,
}

#[get("/")]
pub fn get_user() -> String {
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
pub async fn create_user(user_input: String) -> String {
    // TODO: ADD MORE EXCEPTION HANDLING
    let mut user: User = serde_json::from_str(&user_input).unwrap();
    user.display_name = user.display_name.trim().to_string();
    user.username = user.username.trim().to_string();

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

    let db_session = database::connect().await.unwrap();
    let users_collection: Collection<User> = database::get_collection(&db_session, "users");
    match users_collection.insert_one(user, None).await {
        Ok(result) => {
            let user_id = result.inserted_id;
            let created_user = users_collection
                .find_one(Some(doc! { "_id": user_id }), None)
                .await
                .unwrap();
            let result = serde_json::to_string(&created_user).unwrap();
            result
        }
        Err(_) => "".to_string(),
    }
}
