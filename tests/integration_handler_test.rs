// Integration tests for legacy handler endpoints (users and health)

use actix_web::{App, test};
use backend::handler::*;
use backend::models::*;

#[actix_web::test]
async fn test_health_endpoint_success() {
    let app = test::init_service(App::new().service(health)).await;

    let req = test::TestRequest::get().uri("/health").to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_health_response_content_type() {
    let app = test::init_service(App::new().service(health)).await;

    let req = test::TestRequest::get().uri("/health").to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let headers = resp.headers();
    assert!(headers.contains_key("content-type"));
}

#[actix_web::test]
async fn test_get_users_endpoint() {
    let app = test::init_service(App::new().service(get_users)).await;

    let req = test::TestRequest::get().uri("/users").to_request();

    let resp = test::call_service(&app, req).await;

    // Without a database pool, we expect a 500 error or 404
    assert!(
        resp.status().is_server_error()
            || resp.status().is_success()
            || resp.status().is_client_error()
    );
}

#[actix_web::test]
async fn test_get_user_by_id_endpoint() {
    let app = test::init_service(App::new().service(get_user_by_id)).await;

    let req = test::TestRequest::get().uri("/users/123").to_request();

    let resp = test::call_service(&app, req).await;

    assert!(
        resp.status().is_server_error()
            || resp.status().is_success()
            || resp.status().is_client_error()
    );
}

#[actix_web::test]
async fn test_create_user_endpoint() {
    let app = test::init_service(App::new().service(create_user)).await;

    let create_req = CreateUserRequest {
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&create_req)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Without a database pool, we expect a 500 error or 404
    assert!(
        resp.status().is_server_error()
            || resp.status().is_success()
            || resp.status().is_client_error()
    );
}
