use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use chrono::{Local, Duration};
use serde::{Deserialize, Serialize};
use actix_web::HttpResponse;
use crate::errors::MyStoreError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub session_id: i32,
    pub exp: i64,
}

pub struct SlimUser {
    pub email: String
}

impl From<Claims> for SlimUser {
    fn from(claims: Claims) -> Self {
        SlimUser {
            email: claims.sub
        }
    }
}

impl Claims {
    fn with_email(email: &str, session_id: i32) -> Self {
        Claims {
            sub: email.into(),
            session_id: session_id,
            exp: (Local::now() + Duration::hours(1)).timestamp()
        }
    }
}

pub fn create_token(email: &str, session_id: i32) -> Result<String, MyStoreError> {
    let secret = dotenv!("JWT_SECRET").as_bytes();
    let claims = Claims::with_email(email, session_id);
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
        .map_err( |e| MyStoreError::TokenError(e.to_string()))
}

pub fn decode_token(token: &str) -> Result<Claims, MyStoreError> {
    let secret = dotenv!("JWT_SECRET").as_bytes();
    decode::<Claims>(&token, &DecodingKey::from_secret(secret), &Validation::default())
        .map( |data| data.claims.into())
        .map_err(|e| MyStoreError::TokenError(e.to_string()))
}