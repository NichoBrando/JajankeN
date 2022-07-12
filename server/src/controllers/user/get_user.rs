use crate::models::user::User;
use crate::services::database;
use mongodb::bson::doc;
use serde_json;

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
