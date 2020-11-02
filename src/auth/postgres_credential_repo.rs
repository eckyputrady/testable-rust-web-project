use super::ports::*;
use sqlx::PgPool;
use async_trait::async_trait;
use sqlx::prelude::PgQueryAs;
use std::sync::Arc;

pub struct PostgresCredentialRepoImpl {
    pub pg_pool: Arc<PgPool>
}

#[async_trait]
impl CredentialRepo for PostgresCredentialRepoImpl {
    async fn save_credential(self: &Self, credential: &Credential) -> bool {
        sqlx::query("insert into credentials (username, password) values ($1, crypt($2, gen_salt('bf')))")
            .bind(&credential.username)
            .bind(&credential.password)
            .execute(&*self.pg_pool)
            .await
            .map(|row| row > 0)
            .unwrap_or(false)
    }

    async fn is_credential_exists(self: &Self, credential: &Credential) -> bool {
        let (found,): (bool,) = sqlx::query_as("select true from credentials where username = $1 and password = crypt($2, password)")
            .bind(&credential.username)
            .bind(&credential.password)
            .fetch_one(&*self.pg_pool)
            .await
            .unwrap_or((false,));
        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use std::sync::Arc;

    #[actix_web::main]
    #[test]
    async fn test_save_and_check() {
        let pg_pool = PgPool::builder()
            .build("postgresql://postgres:test@localhost:5431")
            .await
            .expect("Unable to connect to DB");
        sqlx::query("drop database if exists test_credential_repo").execute(&pg_pool).await.unwrap();
        sqlx::query("create database test_credential_repo").execute(&pg_pool).await.unwrap();
        let pg_pool = crate::infrastructure::postgresql::configure_with_db_url("postgresql://postgres:test@localhost:5431/test_credential_repo").await;

        let sut = PostgresCredentialRepoImpl { pg_pool: Arc::new(pg_pool) };

        let credential = Credential { username: "u".to_string(), password: "p".to_string() };

        assert_eq!(false, sut.is_credential_exists(&credential).await);

        assert_eq!(true, sut.save_credential(&credential).await);
        assert_eq!(true, sut.is_credential_exists(&credential).await);
    }
}