use crate::models::user::{UserInfo};
use crate::services::database;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::options::FindOneOptions;
use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/<id>")]
pub async fn get(id: String) -> Result<Json<UserInfo>, Status> {
    let client = database::connect().await.unwrap();
    let users_collection: Collection<UserInfo> = database::get_collection(&client, "users");
    let user_project = doc! {
        "display_name": 1,
        "image_url": 1
    };
    let find_one_options = FindOneOptions::builder().projection(user_project).build();
    let user: Option<UserInfo> = users_collection.find_one(doc! {"_id": database::get_object_id(&id)}, find_one_options).await.unwrap();
    match user {
        Some(mut user) => {
            match user._id {
                Some(id) => {
                    let id_string = database::get_object_id_string(&id);
                    user.id = Some(id_string);
                }
                None => {}
            }
            user._id = None;
            Ok(Json(user))
        },
        None => {
            Err(Status::NotFound)
        },
    }
}
