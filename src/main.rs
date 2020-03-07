
pub mod schema;
pub mod models;
pub mod db;
pub mod handlers;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
use actix_web::{HttpServer, App, web, HttpRequest, HttpResponse, middleware};

embed_migrations!("./migrations");

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let connection = db::establish_connection();
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout());
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/products")
                .route(web::get().to(handlers::get_list)))
            .service(
                web::resource("/new_product")
                .route(web::post().to(handlers::insert)))
            .service(
                web::resource("/")
                .route(web::get().to(handlers::index)))
            .service(
                web::resource("/product/{id}")
                .route(web::get().to(handlers::get_one))
                .route(web::delete().to(handlers::delete))
                .route(web::patch().to(handlers::update))
            )
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}