use super::ports::*;
use async_trait::async_trait;

pub struct AuthServiceImpl<A: CredentialRepo, B: TokenRepo> {
    pub credential_repo: A,
    pub token_repo: B,
    pub metrics: std::sync::Arc<Metrics>
}

#[async_trait]
#[metered::metered(registry = Metrics, visibility = pub)]
#[measure([HitCount, ResponseTime])]
impl <A, B> AuthService for AuthServiceImpl<A, B>
    where A: CredentialRepo + Sync + Send,
          B: TokenRepo + Sync + Send {

    async fn register(self: &Self, credential: &Credential) -> bool {
        self.credential_repo.save_credential(credential).await
    }

    async fn login(self: &Self, credential: &Credential) -> Option<Token> {
        if !self.credential_repo.is_credential_exists(credential).await {
            return None;
        }

        let token = self.token_repo.generate_token().await;
        if !self.token_repo.save_token(&token, &credential.username).await {
            return None;
        }

        Some(token)
    }

    async fn authenticate(self: &Self, token: &Token) -> Option<String> {
        self.token_repo.get_username_by_token(token).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[actix_web::main]
    #[test]
    async fn test_login_success() {
        let credential = Credential { username: "u".to_string(), password: "p".to_string() };
        let token = "token".to_string();

        let mut credential_repo = MockCredentialRepo::new();
        credential_repo.expect_is_credential_exists()
            .with(eq(credential.clone()))
            .return_const(true);

        let mut token_repo = MockTokenRepo::new();
        token_repo.expect_generate_token()
            .return_const(token.clone());
        token_repo.expect_save_token()
            .with(eq(token.clone()), eq(credential.username.clone()))
            .return_const(true);

        let sut = AuthServiceImpl { credential_repo, token_repo };

        let actual = sut.login(&credential).await;
        let expected = Some(token.clone());
        assert_eq!(expected, actual);
    }

    #[actix_web::main]
    #[test]
    async fn test_login_failure_unable_to_save_token() {
        let credential = Credential { username: "u".to_string(), password: "p".to_string() };
        let token = "token".to_string();

        let mut credential_repo = MockCredentialRepo::new();
        credential_repo.expect_is_credential_exists()
            .with(eq(credential.clone()))
            .return_const(true);

        let mut token_repo = MockTokenRepo::new();
        token_repo.expect_generate_token()
            .return_const(token.clone());
        token_repo.expect_save_token()
            .with(eq(token.clone()), eq(credential.username.clone()))
            .return_const(false);

        let sut = AuthServiceImpl { credential_repo, token_repo };

        let actual = sut.login(&credential).await;
        let expected = None;
        assert_eq!(expected, actual);
    }

    #[actix_web::main]
    #[test]
    async fn test_login_failure_credential_does_not_exists() {
        let credential = Credential { username: "u".to_string(), password: "p".to_string() };

        let mut credential_repo = MockCredentialRepo::new();
        credential_repo.expect_is_credential_exists()
            .with(eq(credential.clone()))
            .return_const(false);

        let token_repo = MockTokenRepo::new();

        let sut = AuthServiceImpl { credential_repo, token_repo };

        let actual = sut.login(&credential).await;
        let expected = None;
        assert_eq!(expected, actual);
    }
}