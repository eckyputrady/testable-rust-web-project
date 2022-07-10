use super::ports::*;
use actix_web::{web, Responder};
use actix_web::web::Json;

pub fn configure<T: 'static + AuthService>(service: web::Data<T>, cfg: &mut web::ServiceConfig) {
    cfg.app_data(service);
    cfg.route("/register", web::post().to(register::<T>));
    cfg.route("/login", web::post().to(login::<T>));
    cfg.route("/authenticate", web::post().to(authenticate::<T>));
}

async fn register<T: AuthService>(service: web::Data<T>, body: Json<Credential>) -> impl Responder {
    Json(service.register(&body).await)
}

async fn login<T: AuthService>(service: web::Data<T>, body: Json<Credential>) -> impl Responder {
    Json(service.login(&body).await)
}

async fn authenticate<T: AuthService>(service: web::Data<T>, body: Json<String>) -> impl Responder {
    Json(service.authenticate(&body).await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_authenticate_wrong() {
        let mut auth_service = MockAuthService::new();
        auth_service.expect_authenticate().return_const(None);
        let auth_service = web::Data::new(auth_service);

        let mut sut = test::init_service(App::new().configure(|cfg| configure(auth_service, cfg))).await;

        let req = test::TestRequest::post()
            .uri("/authenticate")
            .set_json(&"test")
            .to_request();
        let resp = test::call_service(&mut sut, req).await;
        let actual_body: Option<String> = test::read_body_json(resp).await;
        assert_eq!(actual_body, None);
    }

    #[tokio::test]
    async fn test_authenticate_correct() {
        let mut auth_service = MockAuthService::new();
        auth_service.expect_authenticate()
            .with(eq("test".to_string()))
            .return_const(Some("username".to_string()));
        let auth_service = web::Data::new(auth_service);

        let mut sut = test::init_service(App::new().configure(|cfg| configure(auth_service, cfg))).await;

        let req = test::TestRequest::post()
            .uri("/authenticate")
            .set_json(&"test")
            .to_request();
        let resp = test::call_service(&mut sut, req).await;
        let actual_body: Option<String> = test::read_body_json(resp).await;
        assert_eq!(actual_body, Some("username".to_string()));
    }
}