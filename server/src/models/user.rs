use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub display_name: String,
    pub image_url: Option<String>,
    pub username: String,
    pub password: Option<String>,
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub display_name: String,
    pub image_url: Option<String>,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        UserInfo {
            display_name: user.display_name,
            image_url: user.image_url,
        }
    }
}
