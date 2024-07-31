use async_trait::async_trait;

#[async_trait]
pub trait Hasher: Send + Sync {
    async fn hash<T: Into<String>>(&self, value: T) -> String;
    async fn verify<T: Into<String>>(&self, value: T, hash: T) -> bool;
}