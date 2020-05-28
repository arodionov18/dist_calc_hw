use actix_web::web;
use actix_web::{HttpResponse, HttpRequest};
use crate::models::{ User, RegisterUser, AuthUser, Tokens };
use crate::jwt;
use crate::errors::MyStoreError;


pub async fn register(new_user: web::Json<RegisterUser>, req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    let register_user = new_user
        .into_inner()
        .validates()
        .map_err( |e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })?;
    User::create(register_user)
        .map( |user| HttpResponse::Ok().json(user))
        .map_err( |e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub async fn login(auth_user: web::Json<AuthUser>, req: HttpRequest)
    -> Result<HttpResponse, HttpResponse> {

    let tokens = auth_user.login()
                        .map_err(|e| {
                            match e {
                                MyStoreError::DBError(diesel::result::Error::NotFound) =>
                                    HttpResponse::NotFound().json(e.to_string()),
                                _ =>
                                    HttpResponse::InternalServerError().json(e.to_string())
                            }
                        })?;

    Ok(HttpResponse::Ok().json(tokens))
}

pub async fn refresh(tokens: web::Json<Tokens>, req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    //use std::ops::Deref;
    let new_tokens = Tokens::refresh(tokens.into_inner())
                        .map_err(|e| {
                            HttpResponse::InternalServerError().json(e.to_string())
                        })?;
    Ok(HttpResponse::Ok().json(new_tokens))
}

pub async fn validate(req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    let tokens = Tokens {
        access: serde::export::Some(req.headers().get("Auth").map_or_else(|| "", |v| v.to_str().unwrap()).to_string()),
        refresh: None
    };
    let result = tokens.validate()
                    .map_err(|error| {
                            HttpResponse::InternalServerError().json(error.to_string())
                    })?;
    Ok(HttpResponse::Ok().json(tokens))
}
