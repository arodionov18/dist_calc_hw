extern crate lettre;
extern crate lettre_email;
extern crate amiquip;
extern crate dotenv;
extern crate serde_json;

use lettre::{SmtpTransport, SmtpClient, SendableEmail, Envelope, EmailAddress, Transport};
use lettre::stub::StubTransport;
use lettre_email::EmailBuilder;
use std::path::Path;
use amiquip::{Connection, Exchange, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};

fn send_email(address: &str, text: &str) -> Result<(), ()> {
    let address_from = std::env::var("FROM_ADDRESS").unwrap_or_else(|_| String::from("arodionov@gmail.com"));
    let email = EmailBuilder::new()
                .to(String::from(address))
                .from(address_from)
                .subject("Confirm your address")
                .text(text)
                .build()
                .map_err(|e| {
                    log::error!("Failed to parse email: {}", e.to_string());
                });
    
    let email = match email {
        Ok(email) => email,
        Err(()) => return Ok(()),
    };

    let domain = std::env::var("SMTP_SERVER").unwrap_or_else(|_| String::from("localhost:25"));

    let is_mocked = std::env::var("IS_MOCKED").unwrap_or_else(|_| String::from("true"));
    
    if is_mocked == "true" {
        let mut mailer = StubTransport::new_positive();
        let result = mailer.send(email.into());

        if result.is_ok() {
            println!("Email sent");
            Ok(())
        } else {
            println!("Could not send email: {:?}", result);
            Err(())
        }
    } else {
        let mut mailer = SmtpClient::new_simple(&domain).unwrap().transport();
        let result = mailer.send(email.into());

        if result.is_ok() {
            println!("Email sent");
            Ok(())
        } else {
            println!("Could not send email: {:?}", result);
            Err(())
        }
    }
}

fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    let amqp_url = std::env::var("RABBITMQ_URL").unwrap_or_else(|_| String::from("amqp://guest:guest@localhost:5672"));

    log::info!("RABBIT_URL:{}", amqp_url);

    let mut connection = Connection::insecure_open(&amqp_url)?;
    
    let channel = connection.open_channel(None)?;

    log::info!("Opened channel");

    let queue_name = std::env::var("QUEUE")
                     .unwrap_or_else(|_| String::from("notifications"));
    let queue = channel.queue_declare(queue_name, QueueDeclareOptions::default())?;

    // Start a consumer.
    let consumer = queue.consume(ConsumerOptions::default())?;
    println!("Waiting for messages. Press Ctrl-C to exit.");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body: serde_json::Value = serde_json::from_str(&String::from_utf8_lossy(&delivery.body)).unwrap();
                println!("({:>3}) Received [{}]", i, body);
                let email = body["email"].as_str().unwrap();
                let text = body["text"].as_str().unwrap();
                log::info!("Parsed values");
                if send_email(email, text).is_ok() {
                    consumer.ack(delivery)?;
                }
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}
