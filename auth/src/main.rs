pub mod schema;
pub mod models;
pub mod db;
pub mod handlers;
pub mod jwt;
pub mod errors;
pub mod confirm;

extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;
extern crate libreauth;

extern crate amiquip;

use actix_web::{HttpServer, App, web, middleware};
use db::establish_connection;

embed_migrations!("./migrations");

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| String::from("0.0.0.0:8089"));
    let connection = db::establish_connection();
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(establish_connection())
            .service(
                web::resource("/auth")
                .route(web::post().to(handlers::login)))
            .service(
                web::resource("/register")
                .route(web::post().to(handlers::register)))
            .service(
                web::resource("/refresh")
                .route(web::post().to(handlers::refresh))
            )
            .service(
                web::resource("/validate")
                .route(web::post().to(handlers::validate))
            )
            .service(
                web::resource("/confirm/{token}")
                .route(web::patch().to(handlers::confirm))
            )
    })
        .bind(bind_addr)?
        .run()
        .await
}