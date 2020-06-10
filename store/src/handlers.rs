use actix_web::{web, HttpRequest, HttpResponse, http::header::HeaderValue};
use crate::models::{Product, ProductList, NewProduct, ListQuery};
use serde_json::json;
use crate::auth;

pub async fn validate(auth: Option<&HeaderValue>) -> Result<i32, HttpResponse> {
    let response = auth::validate(auth).await;
    return response
}

pub async fn insert(new_product: web::Json<NewProduct>,
                    req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    
    let validation = match validate(req.headers().get("Auth")).await {
        Err(e) => return Err(e),
        Ok(v) => v,
    };
    if validation == 1 {
        return new_product
                    .create()
                    .map(|product| HttpResponse::Ok().json(product))
                    .map_err(|e| {
                        HttpResponse::BadRequest().json(json!( {
                            "status": 404,
                            "message": e.to_string()
                        }))
                    });
    } else {
        return Err(HttpResponse::Unauthorized().json(json!({
            "status": 401,
            "message": "Only for admins"
        })));
    }    
}

pub async fn get_list(web::Query(query): web::Query<ListQuery>,
                      req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    let validation = match validate(req.headers().get("Auth")).await {
        Err(e) => return Err(e),
        Ok(v) => v,
    };
    if validation < 2 {
        return Ok(HttpResponse::Ok().json(json!(ProductList::list(query))));
    } else {
        return Err(HttpResponse::Forbidden().json(json!({
            "status": 403,
            "message": "You should authentificate"
        })));
    }
}

pub async fn get_one(id: web::Path<i32>,
                     req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    let validation = match validate(req.headers().get("Auth")).await {
        Err(e) => return Err(e),
        Ok(v) => v,
    };
    if validation < 2 {
        return Product::find(&id)
                    .map(|product| HttpResponse::Ok().json(product))
                    .map_err(|e| {
                        HttpResponse::BadRequest().json(e.to_string())
                    });
    } else {
        return Err(HttpResponse::Forbidden().json(json!({
            "status": 403,
            "message": "You should authentificate"
        })));
    }
    
}

pub async fn delete(id: web::Path<i32>,
                    req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    let validation = match validate(req.headers().get("Auth")).await {
        Err(e) => return Err(e),
        Ok(v) => v,
    };
    if validation == 1 {
        return Product::delete(&id)
                    .map(|product| HttpResponse::Ok().json(product))
                    .map_err(|e| {
                        HttpResponse::BadRequest().json(e.to_string())
                    });
    } else {
        return Err(HttpResponse::Unauthorized().json(json!({
            "status": 401,
            "message": "Only for admins"
        })));
    } 
}

pub async fn update(id: web::Path<i32>, 
                    new_product: web::Json<NewProduct>,
                    req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    let validation = match validate(req.headers().get("Auth")).await {
        Err(e) => return Err(e),
        Ok(v) => v,
    };
    if validation == 1 {
        return Product::update(&id, &new_product)
                    .map(|product| HttpResponse::Ok().json(product))
                    .map_err(|e| {
                        HttpResponse::BadRequest().json(e.to_string())
                    });
    } else {
        return Err(HttpResponse::Unauthorized().json(json!({
            "status": 401,
            "message": "Only for admins"
        })));
    } 
}

pub async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}