use actix_web::{HttpResponse, HttpRequest, client::Client, http::header::HeaderValue};
use dotenv::dotenv;
use std::env;
use actix_web::http::header;
use futures::future::{Future, lazy};

#[derive(Serialize, Deserialize)]
pub struct ValidationQuery {
    pub access: Option<String>
}

pub async fn validate(head: Option<&HeaderValue>) -> Result<HttpResponse, HttpResponse> {
    dotenv().ok();
    let mut auth_url = env::var("AUTH_URL").unwrap_or_else(|_| String::from("http://0.0.0.0:8089"));
    auth_url += "/validate";
    let mut client = Client::new();
    let response = client.post(auth_url)
                    .header("Auth", head.map_or_else(|| "", |v| v.to_str().unwrap()))
                    .send().await;
    response.map(|r| {
        println!("Response: {:?}, status: {}", &r, &r.status().as_str());
        match r.status().as_str() {
            "200" => HttpResponse::Ok().finish(),
            _ => HttpResponse::Unauthorized().finish()
        }
    })
            .map_err(|e| HttpResponse::Unauthorized().json(e.to_string()))
}