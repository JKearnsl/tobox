use argon2::{Argon2, Params, password_hash::{
    PasswordHash,
    PasswordHasher, PasswordVerifier, rand_core::OsRng, SaltString
}};
use async_trait::async_trait;

use crate::application::common::hasher::Hasher;

pub struct Argon2PasswordHasher {
    hasher: Argon2<'static>
}

impl Argon2PasswordHasher {
    pub fn new() -> Self {
        Self {
            hasher: Argon2::new(
                argon2::Algorithm::Argon2id,
                argon2::Version::V0x13,
                Params::new(
                    2048,
                    64,
                    4,
                    Some(64)
                ).unwrap()
            )
        }
    }
}

#[async_trait]
impl Hasher for Argon2PasswordHasher {
    async fn hash(&self, value: &str) -> String {
        let hasher = self.hasher.clone();
        let value = value.to_owned();
        let hash = tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);
            let hash = hasher.hash_password(
                value.as_bytes(),
                &salt
            ).unwrap();
            hash.to_string()
        }).await.unwrap();
        hash
    }

    async fn verify(&self, value: &str, hash: &str) -> bool {
        let hasher = self.hasher.clone();
        let value = value.to_owned();
        let hash = hash.to_owned();
        let result = tokio::task::spawn_blocking(move || {
            let parsed_hash = PasswordHash::new(&hash).unwrap();
            hasher.verify_password(
                value.as_bytes(),
                &parsed_hash
            ).is_ok()
        }).await.unwrap();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hash() {
        let hasher = Argon2PasswordHasher::new();
        let value = "test";
        let hash = hasher.hash(value).await;
        assert_ne!(hash, value);
    }

    #[tokio::test]
    async fn test_verify() {
        let hasher = Argon2PasswordHasher::new();
        let value = "test";
        let hash = hasher.hash(value).await;
        let result = hasher.verify(value, &hash).await;
        assert_eq!(result, true);
    }
}
