use std::env;

use chrono::{TimeDelta, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::{Builder, Uuid};

use crate::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String,
    exp: usize,
    iat: usize,
    iss: String,
    nbf: usize,
    pub sub: String,
    pub name: String,
}

pub fn jwts_encode(my_claims: Claims) -> Result<std::string::String, jsonwebtoken::errors::Error> {
    encode(&Header::default(), &my_claims, &get_encoding_key())
}
pub fn create_user_token(user: User) -> Result<std::string::String, jsonwebtoken::errors::Error> {
    let week_in_seconds = 3600 * 24 * 7;
    let address = env::var("BASE_URL").unwrap();
    jwts_encode(Claims {
        aud: address.clone(),
        exp: Utc::now()
            .checked_add_signed(TimeDelta::new(week_in_seconds, 0).unwrap())
            .unwrap()
            .timestamp() as usize,
        iat: Utc::now().timestamp() as usize,
        iss: address,
        nbf: Utc::now().timestamp() as usize,
        sub: user.id,
        name: user.username,
    })
}

pub fn jwts_decode(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(token, &get_decoding_key(), &Validation::default())
}

pub fn generate_id(namespace: &str, id: &str) -> String {
    let digest = md5::compute(format!("{}-{}", namespace, id));
    let md5_bytes = digest.0;
    Builder::from_md5_bytes(md5_bytes).into_uuid().to_string()
}

fn get_encoding_key() -> EncodingKey {
    EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref())
}
fn get_decoding_key() -> DecodingKey {
    DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref())
}
