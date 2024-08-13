use chrono::{Duration, Utc};
use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use crate::utils::constants;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub expiry: usize,
    pub iat: usize,
    pub email: String,
    pub id: i32,
}

pub fn encode_jwt(email: String, id: i32) -> Result<String, Error> {
    let now = Utc::now();
    let expire = Duration::hours(24);

    let claims = Claims {
        expiry: (now + expire).timestamp() as usize,
        iat: now.timestamp() as usize,
        email,
        id,
    };

    let secret = (*constants::SECRET).clone();

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, Error> {
    let secret = (*constants::SECRET).clone();
    let claim_data: Result<TokenData<Claims>, Error> = decode(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    );

    claim_data
}