use crate::jwt;
use crate::errors::MyStoreError;
use crate::models::{User};
use actix_web::{HttpResponse};
use crate::db;
use crate::schema::users;
use amiquip::{Connection, Exchange, Publish, Result};

use serde_json::json;

fn create_confirm_link(user: &User) -> Result<String, MyStoreError> {
    let token = jwt::create_confirm_token(&user.email, user.id)?;
    let address = std::env::var("CONFIRM_ADDRESS").unwrap_or_else(|_| String::from("0.0.0.0:8089"));
    Ok(address + "/confirm/" +&token)
}

pub fn make_new_confirmation(user: &User) -> Result <(), MyStoreError> {
    let link = create_confirm_link(user)?;
    // put message in queue
    println!("confirm link created");
    let amqp_url = std::env::var("RABBITMQ_URL").unwrap_or_else(|_| String::from("amqp://guest:guest@localhost:5672"));

    let mut connection = Connection::insecure_open(&amqp_url)?;
    println!("connection set");

    let channel = connection.open_channel(None)?;

    println!("Opened channel");

    let exchange = Exchange::direct(&channel);

    let message = json!({
        "email": &user.email,
        "text": "To confirm registration go to ".to_owned() + &link
    });

    exchange.publish(Publish::new(message.to_string().as_bytes(), "notifications"))?;

    connection.close();

    Ok(())
}

pub fn make_new_confirmation_from_email(user_email: &str) -> Result<(), MyStoreError> {
    use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods};
    use crate::schema::users::dsl::email;

    let connection = db::establish_connection();

    let mut records = users::table
                        .filter(email.eq(&user_email))
                        .load::<User>(&connection)?;

    let user = records
                    .pop()
                    .ok_or(MyStoreError::DBError(diesel::result::Error::NotFound))?;
    
    make_new_confirmation(&user)
}