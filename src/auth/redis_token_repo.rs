use super::ports::*;
use async_trait::async_trait;
use uuid::Uuid;
use redis::{AsyncCommands, RedisError};
use std::sync::Arc;
use tracing::instrument;
use tracing::{debug, error, info, span, warn, trace, Level};
use redis::aio::Connection;

pub struct RedisTokenRepoImpl {
    pub redis_client: Arc<redis::Client>
}

#[async_trait]
impl TokenRepo for RedisTokenRepoImpl {

    async fn generate_token(self: &Self) -> Token {
        Uuid::new_v4().to_string()
    }

    #[instrument(skip(self, token, username))]
    async fn save_token(self: &Self, token: &Token, username: &String) -> bool {
        let redis_client = &*self.redis_client;
        let result = async {
            let mut conn = redis_client.get_async_connection().await?;
            let key = format!("token:{}", token);
            conn.set::<_, _, String>(key, username).await
        }.await;

        match result {
            Err(e) => {
                error!("Failed to store token. {}", e);
                false
            },
            Ok(_) => {
                debug!("Token is stored into Redis");
                true
            }
        }
    }

    async fn get_username_by_token(self: &Self, token: &Token) -> Option<String> {
        let redis_client = &*self.redis_client;
        if let Ok(mut conn) = redis_client.get_async_connection().await {
            let key = format!("token:{}", token);
            conn.get(key).await.ok()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[actix_web::main]
    #[test]
    async fn test_save_and_check() {
        let redis_client = redis::Client::open("redis://localhost:6378").unwrap();
        let sut = RedisTokenRepoImpl { redis_client: Arc::new(redis_client) };

        let token = sut.generate_token().await;
        let username = "username".to_string();
        assert_eq!(None, sut.get_username_by_token(&token).await);
        assert_eq!(true, sut.save_token(&token, &username).await);
        assert_eq!(Some(username), sut.get_username_by_token(&token).await);
    }
}