use actix_web::{web, HttpRequest, HttpResponse};
use crate::models::{Product, ProductList, NewProduct, ListQuery};
use serde_json::json;

pub async fn insert(new_product: web::Json<NewProduct>, req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    new_product
        .create()
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| {
            HttpResponse::BadRequest().json(e.to_string())
        })
}

pub async fn get_list(web::Query(query): web::Query<ListQuery>) -> HttpResponse {
    HttpResponse::Ok().json(json!(ProductList::list(query)))
}

pub async fn get_one(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    Product::find(&id)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| {
            HttpResponse::BadRequest().json(e.to_string())
        })
}

pub async fn delete(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    Product::delete(&id)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| {
            HttpResponse::BadRequest().json(e.to_string())
        })
}

pub async fn update(id: web::Path<i32>, new_product: web::Json<NewProduct>) -> Result<HttpResponse, HttpResponse> {
    Product::update(&id, &new_product)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| {
            HttpResponse::BadRequest().json(e.to_string())
        })
}

pub async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}