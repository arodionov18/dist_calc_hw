use actix_web::{web, HttpRequest, HttpResponse, http::header::HeaderValue};
use crate::models::{Product, ProductList, NewProduct, ListQuery};
use serde_json::json;
use crate::auth;

pub async fn validate(auth: Option<&HeaderValue>) -> Result<HttpResponse, HttpResponse> {
    let response = auth::validate(auth).await;
    return response
}

pub async fn insert(new_product: web::Json<NewProduct>,
                    req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    
    let validation = match validate(req.headers().get("Auth")).await {
        Err(e) => return Err(e),
        Ok(v) => v,
    };
    if validation.status().as_str() == "200" {
        return new_product
                    .create()
                    .map(|product| HttpResponse::Ok().json(product))
                    .map_err(|e| {
                        HttpResponse::BadRequest().json(e.to_string())
                    });
    } else {
        return Err(validation);
    }    
}

pub async fn get_list(web::Query(query): web::Query<ListQuery>,
                      req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    let validation = match validate(req.headers().get("Auth")).await {
        Err(e) => return Err(e),
        Ok(v) => v,
    };
    if validation.status().as_str() == "200" {
        return Ok(HttpResponse::Ok().json(json!(ProductList::list(query))));
    } else {
        return Err(validation);
    }
}

pub async fn get_one(id: web::Path<i32>,
                     req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    let validation = match validate(req.headers().get("Auth")).await {
        Err(e) => return Err(e),
        Ok(v) => v,
    };
    if validation.status().as_str() == "200" {
        return Product::find(&id)
                    .map(|product| HttpResponse::Ok().json(product))
                    .map_err(|e| {
                        HttpResponse::BadRequest().json(e.to_string())
                    });
    } else {
        return Err(validation);
    }
    
}

pub async fn delete(id: web::Path<i32>,
                    req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    let validation = match validate(req.headers().get("Auth")).await {
        Err(e) => return Err(e),
        Ok(v) => v,
    };
    if validation.status().as_str() == "200" {
        return Product::delete(&id)
                    .map(|product| HttpResponse::Ok().json(product))
                    .map_err(|e| {
                        HttpResponse::BadRequest().json(e.to_string())
                    });
    } else {
        return Err(validation);
    }
}

pub async fn update(id: web::Path<i32>, 
                    new_product: web::Json<NewProduct>,
                    req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    let validation = match validate(req.headers().get("Auth")).await {
        Err(e) => return Err(e),
        Ok(v) => v,
    };
    if validation.status().as_str() == "200" {
        return Product::update(&id, &new_product)
                    .map(|product| HttpResponse::Ok().json(product))
                    .map_err(|e| {
                        HttpResponse::BadRequest().json(e.to_string())
                    });
    } else {
        return Err(validation);
    }
}

pub async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}