use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};

pub fn create_jwt (user_id: &String) -> String {
    let jwt = encode(
        &Header::default(), 
        &user_id, 
        &EncodingKey::from_secret("ARABNA".to_string().as_bytes())
    );
    jwt.unwrap()
}

pub fn get_user_id_from_jwt (token: &String) -> String {
    let token_data = decode::<String>(
        &token, 
        &DecodingKey::from_secret("ARABNA".to_string().as_bytes()), 
        &Validation::default()
    ).unwrap();
    token_data.claims
}