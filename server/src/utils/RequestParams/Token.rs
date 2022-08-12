use crate::services::auth::users_sessions::{UsersSessions, get_logged_user_id};
use rocket::request::{Outcome, Request, FromRequest};
use rocket::http::Status;
use rocket::State;
use std::convert::Into;

#[derive(Debug)]
pub struct TokenParam(String);

#[derive(Debug)]
pub enum TokenError {   
    Missing,
    Invalid
}

impl From<String> for TokenParam {
    fn from(token: String) -> Self {
        TokenParam(token)
    }
}

impl Into<String> for TokenParam {
    fn into(self) -> String {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TokenParam {
    type Error = TokenError;

    async fn from_request(
        request: &'r Request<'_>
    ) -> Outcome<Self, Self::Error> {
        let user_sessions_lock = request.guard::<&State<UsersSessions>>().await.unwrap();
        let headers = request.headers();
        match headers.get_one("Authorization") {
            Some(token) => {
                let token = token.to_string();
                match get_logged_user_id(&user_sessions_lock, &token).await {
                    Ok(user_id) => return Outcome::Success(TokenParam(user_id)),
                    Err(_) => return Outcome::Failure((Status::Unauthorized, TokenError::Invalid))
                }
            },
            None => return Outcome::Failure((Status::BadRequest, TokenError::Missing))
        };
    }
}
