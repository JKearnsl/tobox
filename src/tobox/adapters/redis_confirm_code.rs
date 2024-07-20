use async_trait::async_trait;
use deadpool_redis::Pool;
use rand::Rng;
use redis::AsyncCommands;

use crate::application::common::confirm_code::ConfirmCode;

pub struct RedisConfirmCode {
    redis: Box<Pool>,
    confirm_code_ttl: u32,
}

impl RedisConfirmCode {
    pub fn new(
        redis: Box<Pool>,
        confirm_code_ttl: u32,
    ) -> Self {
        Self {
            redis,
            confirm_code_ttl,
        }
    }
}

#[async_trait]
impl ConfirmCode for RedisConfirmCode {

    /// **confirm** - метод подтверждения кода.
    ///
    /// Пользователю дается 3 попытки на то, чтобы ввести правильный код подтверждения.
    /// Если пользователь превысил лимит попыток, то пользователь дожидается ttl и
    /// запрашивает новый код подтверждения.
    async fn confirm(&self, key: &str, code: u32) -> Result<(), String> {
        let mut redis = self.redis.get().await.unwrap();

        let stored_data: String = match redis.get(key).await.unwrap() {
            Some(data) => data,
            None => return Err("Сначала запросите код подтверждения".to_string())
        };
        let (stored_code, attempts) = stored_data.split_once(':').unwrap();

        if attempts.parse::<u32>().unwrap() >= 3 {
            return Err("Превышено количество попыток".to_string());
        }

        if stored_code.parse::<u32>().unwrap() == code {
            let _: usize = redis.del(key).await.unwrap();
            Ok(())
        } else {
            let attempts = attempts.parse::<u32>().unwrap() + 1;
            let data = format!("{}:{}", stored_code, attempts);

            let ttl: i64 = redis.ttl(key).await.unwrap();
            let _: String = redis.set(key, data).await.unwrap();
            if ttl > 0 {
                let _: i32 = redis.expire(key, ttl).await.unwrap();
            }

            Err("Неверный код".to_string())
        }
    }

    /// **generate** - метод генерации кода.
    ///
    /// Генерируется шестизначный код и в редис записывается код и 
    /// количество попыток равное нулю.
    async fn generate(&self, key: &str) -> Result<u32, String> {
        let mut redis = self.redis.get().await.unwrap();

        let data: Option<String> = redis.get(key).await.unwrap();
        if data.is_some() {
            return Err("Код уже отправлен".to_string());
        }

        let code: u32 = rand::thread_rng().gen_range(100000..=999999);
        let _: String = redis.set(key, format!("{}:0", code)).await.unwrap();
        let _: i32 = redis.expire(key, self.confirm_code_ttl as i64).await.unwrap();
        
        Ok(code)
    }
}