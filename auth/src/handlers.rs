use actix_web::web;
use actix_web::HttpResponse;
use crate::models::{ User, RegisterUser, AuthUser };


pub fn register(new_user: web::Json(RegisterUser)) -> Result<HttpResponse, HttpResponse> {
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

use actix_web::middleware::identity::Identity;
use crate::jwt;
use hex;
use csrf_token::CsrfTokenGenerator;

pub fn login(auth_user: web::Json<AuthUser>,
             id: Identity,
             generator: web::Data<CsrfTokenGenerator>)
    -> Result<HttpResponse, HttpResponse> {

    let user = auth_user.login()
                        .map_err(|e| {
                            match e {
                                MyStoreError::DBError(diesel::result::Error::NotFound) =>
                                    HttpResponse::NotFound().json(e.to_string()),
                                _ =>
                                    HttpResponse::InternalServerError().json(e.to_string())
                            }
                        })?;
    
    let token = jwt::create_token(&user.email)?;
    id.remember(token);

    let response = HttpResponse::Ok()
                    .header("X-CSRF-TOKEN", hex::encode(generator.generate()))
                    .json(user);
    Ok(response)
}

pub fn logout(id: Identity) -> Result<HttpResponse, HttpResponse> {
    id.forget();
    Ok(HttpResponse::Ok().into())
}