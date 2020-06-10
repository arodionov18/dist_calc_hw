use actix_web::web;
use actix_web::{HttpResponse, HttpRequest};
use crate::models::{ User, RegisterUser, AuthUser, Tokens };
use crate::jwt;
use crate::errors::MyStoreError;
use crate::confirm::{make_new_confirmation, make_new_confirmation_from_email};

pub async fn confirm(token: web::Path<String>, req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    log::info!("Confirm handler");
    println!("Confirm handler");
    let confirm_info = jwt::decode_confirm_token(&token)
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string() + "LOH")
        })?;
    println!("AHAHA");
    User::make_confirmation(confirm_info).map_err( |e| {
            HttpResponse::InternalServerError().json(e.to_string() + "confirmation")
        })?;
    Ok(HttpResponse::Ok().json("Account confirmed"))
}

pub async fn register(new_user: web::Json<RegisterUser>, req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    let register_user = new_user
        .into_inner()
        .validates()
        .map_err( |e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })?;
    let user = User::create(register_user)
        .map_err( |e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })?;
    
    make_new_confirmation(&user)
        .map_err( |e| {
            HttpResponse::BadRequest().json(e.to_string())
        })?;
    
    Ok(HttpResponse::Ok().json(user))
}

pub async fn login(auth_user: web::Json<AuthUser>, req: HttpRequest)
    -> Result<HttpResponse, HttpResponse> {

    let tokens = auth_user.login()
                        .map_err(|e| {
                            match e {
                                MyStoreError::DBError(diesel::result::Error::NotFound) =>
                                    HttpResponse::NotFound().json(e.to_string()),
                                MyStoreError::NotConfirmed(_) =>
                                    make_new_confirmation_from_email(&auth_user.email).map(
                                        |_| HttpResponse::NotFound().json("Confirmation link sent again")
                                    ).map_err(|e| HttpResponse::InternalServerError().json(e.to_string())).unwrap(),
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

use tonic::{transport::Server, Request, Response, Status};
use pb::authentificator_server::{Authentificator, AuthentificatorServer};
use pb::{VerifyRequest, VerifyResponse, AdminRights};

#[derive(Debug, Default)]
pub struct MyAuthentificator {}

#[tonic::async_trait]
impl Authentificator for MyAuthentificator {
    async fn verify(
        &self,
        request: Request<VerifyRequest>) -> Result<Response<VerifyResponse>, Status> {
        log::info!("Got a request: {:?}", request);

        let tokens = Tokens {
            access: Some(request.into_inner().token),
            refresh: None,
        };

        let result = tokens.validate()
                        .map_err(|error| {
                            Status::unauthenticated("")
                        })?;

        let reply = pb::VerifyResponse {
            result: result,
        };

        Ok(Response::new(reply))
    }
}

pub async fn set_role(email: web::Json<String>, req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    let tokens = Tokens {
        access: serde::export::Some(req.headers().get("Auth").map_or_else(|| "", |v| v.to_str().unwrap()).to_string()),
        refresh: None
    };

    let result = tokens.validate()
                    .map_err(|error| {
                        HttpResponse::Forbidden().json(json!({
                            "status": 403,
                            "message": "You should authentificate",
                            "error": error.to_string()
                        }))
                    })?;

    if result != 1 {
        return Err(HttpResponse::Forbidden().json(json!({
            "status": 403,
            "message": "You should be admin"
        })));
    }

    let smth = crate::models::set_role(&email).map_err(|error| {
        HttpResponse::BadRequest().json(json!({
            "status": 402,
            "error": error.to_string()
        }))
    });

    Ok(HttpResponse::Ok().json(json!({
        "status": 200,
        "message": std::format!("user {} now is admin", email)
    })))
}

/*pub async fn validate(req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    let tokens = Tokens {
        access: serde::export::Some(req.headers().get("Auth").map_or_else(|| "", |v| v.to_str().unwrap()).to_string()),
        refresh: None
    };
    println!("Gmm");
    let result = tokens.validate()
                    .map_err(|error| {
                            HttpResponse::InternalServerError().json(error.to_string())
                    })?;
    println!("Success");
    Ok(HttpResponse::Ok().json(tokens))
}*/
