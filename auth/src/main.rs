pub mod schema;
pub mod models;
pub mod db;
pub mod handlers;
pub mod jwt;
pub mod errors;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
use actix_web::{HttpServer, App, web, middleware};

embed_migrations!("./migrations");

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| String::from("0.0.0.0:8089"));
    let csrf_token_header = header::HeaderName::from_lowercase(b"x-csrf-token").unwrap();
    let connection = db::establish_connection();
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap( 
                IdentityService::new(
                    CookieIdentityPolicy::new(dotenv!("SECRET_KEY").as_bytes())
                        .domain(dotenv!("MYSTOREDOMAIN"))
                        .name("mystorejwt")
                        .path("/")
                        .max_age(Duration::days(1).num_seconds())
                        .secure(dotenv!("COOKIE_SECURE").parse().unwrap())
                )
            )
            .wrap(
                cors::Cors::new()
                    .allowed_origin(dotenv!("ALLOWED_ORIGIN"))
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION,
                                          header::CONTENT_TYPE,
                                          header::ACCEPT,
                                          csrf_token_header.clone()])
                    .expose_headers(vec![csrf_token_header.clone()])
                    .max_age(3600)
            )
            .data(
                CsrfTokenGenerator::new(
                    dotenv!("CSRF_TOKEN_KEY").as_bytes().to_vec(),
                    Duration::hours(1)
                )
            )
            .data(establish_connection())
            .service(
                web::resource("/auth")
                .route(web::post().to(handlers::login)))
                .route(web::delete().to(handlers::logout)))
            .service(
                web::resource("/register")
                .route(web::post().to(handlers::register)))
    })
        .bind(bind_addr)?
        .run()
        .await
}