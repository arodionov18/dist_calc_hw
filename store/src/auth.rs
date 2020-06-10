use actix_web::{HttpResponse, HttpRequest, client::Client, http::header::HeaderValue};
use dotenv::dotenv;
use std::env;
use actix_web::http::header;
use futures::future::{Future, lazy};

use pb::authentificator_client::AuthentificatorClient;
use pb::{VerifyRequest, VerifyResponse};

#[derive(Serialize, Deserialize)]
pub struct ValidationQuery {
    pub access: Option<String>
}

pub async fn validate(head: Option<&HeaderValue>) -> Result<i32, HttpResponse> {
    dotenv().ok();
    let mut auth_url = env::var("AUTH_URL").unwrap_or_else(|_| String::from("0.0.0.0:50501"));

    let mut client = AuthentificatorClient::connect(auth_url).await.expect("BAD");

    let request = tonic::Request::new(VerifyRequest {
        token: head.map_or_else(|| "", |v| v.to_str().unwrap()).into(),
    });

    let response = client.verify(request).await;

    response.map(|r| {
        return r.into_inner().result
    })
            .map_err(|e| HttpResponse::Unauthorized().json(e.to_string()))
}