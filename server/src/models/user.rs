use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub display_name: String,
    pub image_url: Option<String>,
    pub username: String,
    pub password: Option<String>,
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
}
