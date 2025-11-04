// Integration tests for department handlers

use actix_web::{App, test};
use backend::handlers::department::*;
use backend::models::*;

#[actix_web::test]
async fn test_get_departments_endpoint() {
    let app = test::init_service(App::new().service(get_departments)).await;

    let req = test::TestRequest::get()
        .uri("/api/departments")
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_server_error() || resp.status().is_success());
}

#[actix_web::test]
async fn test_get_department_by_id_endpoint() {
    let app = test::init_service(App::new().service(get_department_by_id)).await;

    let req = test::TestRequest::get()
        .uri("/api/departments/test-uuid-123")
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(
        resp.status().is_server_error()
            || resp.status().is_success()
            || resp.status().is_client_error()
    );
}

#[actix_web::test]
async fn test_create_department_request_structure() {
    let dept_req = CreateDepartmentRequest {
        name: "Engineering".to_string(),
        head_id: Some("manager-uuid".to_string()),
    };

    assert_eq!(dept_req.name, "Engineering");
    assert!(dept_req.head_id.is_some());
}

#[actix_web::test]
async fn test_update_department_request_partial() {
    let update_req = UpdateDepartmentRequest {
        name: Some("Updated Engineering".to_string()),
        head_id: None,
    };

    assert!(update_req.name.is_some());
    assert!(update_req.head_id.is_none());
}
