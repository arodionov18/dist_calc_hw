#[macro_use]
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use crate::models::{Product, ProductList, NewProduct};

pub async fn insert(new_product: web::Json<NewProduct>, req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    new_product
        .create()
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub async fn get_list(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(ProductList::list())
}

pub async fn get_one(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    Product::find(&id)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub async fn delete(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    Product::delete(&id)
        .map(|_| HttpResponse::Ok().json("Succeded deletion"))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub async fn update(id: web::Path<i32>, new_product: web::Json<NewProduct>) -> Result<HttpResponse, HttpResponse> {
    Product::update(&id, &new_product)
        .map(|_| HttpResponse::Ok().json("Succeded update"))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}