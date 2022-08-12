use std::collections::HashMap;

use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use serde::{Serialize, Deserialize};
use chrono::{Utc};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: String,
    exp: usize
}

const SECRET_KEY: &[u8] = b"secret";

pub fn create_jwt (user_id: &String) -> String {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(30))
        .expect("valid timestamp")
        .timestamp();
    let jwt = encode(
        &Header::new(Algorithm::HS256),
        &Claims {
            user_id: user_id.to_owned(),
            exp: expiration as usize
        }, 
        &EncodingKey::from_secret(SECRET_KEY)
    );
    jwt.unwrap()
}

pub fn get_user_id_from_jwt (jwt: &String) -> String {
    let jwt_decoded = decode::<Claims>(
        &jwt, 
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::new(Algorithm::HS256)
    );
    match jwt_decoded {
        Ok(decoded) => {
            return decoded.claims.user_id;
        }
        Err(e) => {
            return String::from("");
        }
    }
}