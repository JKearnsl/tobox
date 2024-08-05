use async_trait::async_trait;

use crate::domain::models::session::Session;


#[async_trait]
pub trait SessionReader {
    async fn get_session(&self, token: &String) -> Option<Session>;
}

#[async_trait]
pub trait SessionWriter {
    async fn save_session(&self, data: &Session);
}


pub trait SessionGateway: SessionReader + SessionWriter {}
