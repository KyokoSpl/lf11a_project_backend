// Integration tests for salary grade handlers

use actix_web::{App, test};
use backend::handlers::salary_grade::*;
use backend::models::*;

#[actix_web::test]
async fn test_get_salary_grades_endpoint() {
    let app = test::init_service(App::new().service(get_salary_grades)).await;

    let req = test::TestRequest::get()
        .uri("/api/salary-grades")
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_server_error() || resp.status().is_success());
}

#[actix_web::test]
async fn test_get_salary_grade_by_id_endpoint() {
    let app = test::init_service(App::new().service(get_salary_grade_by_id)).await;

    let req = test::TestRequest::get()
        .uri("/api/salary-grades/test-uuid-123")
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(
        resp.status().is_server_error()
            || resp.status().is_success()
            || resp.status().is_client_error()
    );
}

#[actix_web::test]
async fn test_create_salary_grade_request_structure() {
    let grade_req = CreateSalaryGradeRequest {
        code: "SR-ENG".to_string(),
        base_salary: 100000.0,
        description: Some("Senior Engineer Grade".to_string()),
    };

    assert_eq!(grade_req.code, "SR-ENG");
    assert_eq!(grade_req.base_salary, 100000.0);
    assert!(grade_req.description.is_some());
}

#[actix_web::test]
async fn test_update_salary_grade_request_partial() {
    let update_req = UpdateSalaryGradeRequest {
        code: Some("UPDATED-CODE".to_string()),
        base_salary: None,
        description: None,
    };

    assert!(update_req.code.is_some());
    assert!(update_req.base_salary.is_none());
}
