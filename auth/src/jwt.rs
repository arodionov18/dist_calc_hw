use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use chrono::{Local, Duration}
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct SlimUser {
    pub email: String
}

impl From<Claims> for SlimUser {
    fn from(claims: Claims) -> self {
        SlimUser {
            email: claims.sub
        }
    }
}

impl Claims {
    fn with_email(email: &str) -> Self {
        Claims {
            sub: email.into(),
            exp: (Local::now() + Duration::hours(1)).timestamp() as usize
        }
    }
}

pub fn create_token(email: &str) -> Result<String, HttpResponse> {
    let secret = dotenv!("JWT_SECRET").as_bytes
    let claims = Claims::with_email(email);
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
        .map_err( |e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn decode_token(token: &str) -> Result<SlimUser, HttpResponse> {
    let secret = dotenv!("JWT_SECRET").as_bytes
    decode::<Claims>(&token, &DecodingKey::from_secret(secret), &Validation::default())
        .map( |data| data.claims.into())
        .map_err(|e| HttpResponse::Unauthorized().json(e.to_string()))
}
