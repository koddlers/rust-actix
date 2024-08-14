use crate::utils::constants;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpMessage, HttpRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::future;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
    pub id: i32,
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> future::Ready<Result<Claims, actix_web::Error>> {
        match req.extensions().get::<Claims>() {
            None => future::ready(Err(actix_web::error::ErrorBadRequest("Bad Claim"))),
            Some(claim) => future::ready(Ok(claim.clone()))
        }
    }
}

pub fn encode_jwt(email: String, id: i32) -> Result<String, Error> {
    let now = Utc::now();
    let expire = Duration::hours(24);

    let claims = Claims {
        exp: (now + expire).timestamp() as usize,
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