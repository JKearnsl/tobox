use async_trait::async_trait;

#[async_trait]
pub trait Hasher: Send + Sync {
    async fn hash(&self, value: &str) -> String;
    async fn verify(&self, value: &str, hash: &str) -> bool;
}