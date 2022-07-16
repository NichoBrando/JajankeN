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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        UserInfo {
            id: match user.id {
                Some(id) => Some(id.to_string()),
                None => None,
            },
            _id: match user.id {
                Some(id) => Some(id),
                None => None,
            },
            display_name: user.display_name,
            image_url: user.image_url,
        }
    }
}
