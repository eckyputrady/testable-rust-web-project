use async_trait::async_trait;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Credential {
    pub username: String,
    pub password: String
}

pub type Token = String;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait AuthService {
    async fn register(&self, credential: &Credential) -> bool;
    async fn login(&self, credential: &Credential) -> Option<Token>;
    async fn authenticate(&self, token: &Token) -> Option<String>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CredentialRepo {
    async fn save_credential(&self, credential: &Credential) -> bool;
    async fn is_credential_exists(&self, credential: &Credential) -> bool;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TokenRepo {
    async fn generate_token(&self) -> Token;
    async fn save_token(&self, token: &Token, username: &String) -> bool;
    async fn get_username_by_token(&self, token: &Token) -> Option<String>;
}