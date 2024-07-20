use std::collections::BTreeMap;

use async_trait::async_trait;
use lapin::protocol::basic::AMQPProperties;
use lapin::types::{AMQPValue, FieldTable, LongString, ShortString};
use serde_json::Value;
use tera::{Context, Tera};

use crate::application::common::email_sender::EmailSender;

pub struct RMQEmailSender {
    rmq_connection: Box<lapin::Connection>,
    exchange: String,
    sender_id: String,
    service_text_id: String,
    tera: Tera
}

impl RMQEmailSender {
    pub fn new(
        rmq_connection: Box<lapin::Connection>,
        exchange: String,
        sender_id: String,
        service_text_id: String,
        tera: Tera
    ) -> Self {
        Self {
            rmq_connection,
            exchange,
            sender_id,
            service_text_id,
            tera
        }
    }
}

#[async_trait]
impl EmailSender for RMQEmailSender {
    
    async fn send(
        &self, 
        to: &str, 
        subject: &str, 
        content: &str, 
        content_type: &str, 
        priority: u8, 
        ttl: u32
    ) {
        let channel = self.rmq_connection.create_channel().await.unwrap();

        channel.exchange_declare(
            &self.exchange,
            lapin::ExchangeKind::Direct,
            lapin::options::ExchangeDeclareOptions {
                durable: true,
                auto_delete: false,
                internal: false,
                passive: true,
                ..Default::default()
            },
            FieldTable::default()
        ).await.unwrap();

        let headers= FieldTable::from(
            BTreeMap::from([
                (ShortString::from("To"), AMQPValue::LongString(LongString::from(to))),
                (ShortString::from("Subject"), AMQPValue::LongString(LongString::from(subject))),
                (ShortString::from("FromId"), AMQPValue::LongString(LongString::from(self.sender_id.clone()))),
            ])
        );

        channel.basic_publish(
            &self.exchange,
            "",
            lapin::options::BasicPublishOptions::default(),
            content.as_bytes(),
            AMQPProperties::default()
                .with_headers(headers)
                .with_timestamp(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
                .with_message_id(ShortString::from(uuid::Uuid::new_v4().to_string()))
                .with_app_id(ShortString::from(self.service_text_id.clone()))
                .with_priority(priority)
                .with_content_type(ShortString::from(content_type))
                .with_expiration(ShortString::from(ttl.to_string()))
        ).await.unwrap();
    }

    async fn send_template(
        &self, 
        to: &str, 
        subject: &str, 
        template: &str, 
        data: Option<BTreeMap<String, Value>>, 
        priority: u8, 
        ttl: u32
    ) {
        
        let context = match data {
            Some(data) => { 
                let mut context = Context::new();
                for (key, value) in data {
                    context.insert(key, &value);
                }
                context
            },
            None => Context::new()
        };
        
        let content = self.tera.render(template, &context).unwrap();
        self.send(to, subject, &content, "text/html", priority, ttl).await;
    }
}