use async_trait::async_trait;
use sha2::{Digest, Sha256};

use crate::application::common::hasher::Hasher;

pub struct Sha256SessionHasher {}


#[async_trait]
impl Hasher for Sha256SessionHasher {
    
    async fn hash(&self, value: &str) -> String {
        let mut hasher = Sha256::new();
        let value = value.to_owned();
        let hash = tokio::task::spawn_blocking(move || {
            hasher.update(value.as_bytes());
            format!("{:x}", hasher.finalize())
        }).await.unwrap();
        hash
    }

    async fn verify(&self, value: &str, hash: &str) -> bool {
        let mut hasher = Sha256::new();
        let value = value.to_owned();
        let hash = hash.to_owned();
        let result = tokio::task::spawn_blocking(move || {
            hasher.update(value.as_bytes());
            format!("{:x}", hasher.finalize()) == hash
        }).await.unwrap();
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hash() {
        let hasher = Sha256SessionHasher {};
        let value = "test";
        let hash = hasher.hash(value).await;
        assert_eq!(hash, "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08");
    }

    #[tokio::test]
    async fn test_verify() {
        let hasher = Sha256SessionHasher {};
        let value = "test";
        let hash = "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08";
        let result = hasher.verify(value, hash).await;
        assert_eq!(result, true);
    }
}