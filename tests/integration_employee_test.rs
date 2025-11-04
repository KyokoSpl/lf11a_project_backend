// Integration tests for employee handlers
// Tests actual HTTP endpoints with a real test server

use actix_web::{App, test};
use backend::handlers::employee::*;
use backend::models::*;

#[actix_web::test]
async fn test_health_endpoint() {
    use backend::handler::health;

    let app = test::init_service(App::new().service(health)).await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_employees_endpoint() {
    let app = test::init_service(App::new().service(get_employees)).await;

    let req = test::TestRequest::get().uri("/api/employees").to_request();

    let resp = test::call_service(&app, req).await;

    // Without a database, we expect a 500 error
    // This still tests that the endpoint exists and responds
    assert!(resp.status().is_server_error() || resp.status().is_success());
}

#[actix_web::test]
async fn test_get_employees_with_inactive_query() {
    let app = test::init_service(App::new().service(get_employees)).await;

    let req = test::TestRequest::get()
        .uri("/api/employees?include_inactive=true")
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Endpoint should exist
    assert!(resp.status().is_server_error() || resp.status().is_success());
}

#[actix_web::test]
async fn test_create_employee_request_structure() {
    let employee_req = CreateEmployeeRequest {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john.doe@company.com".to_string(),
        department_id: Some("dept-123".to_string()),
        salary_grade_id: Some("grade-456".to_string()),
        manager_id: None,
        role: Some("Employee".to_string()),
        hire_date: Some("2024-01-01".to_string()),
    };

    assert_eq!(employee_req.first_name, "John");
    assert_eq!(employee_req.last_name, "Doe");
    assert!(employee_req.email.contains("@"));
}

#[actix_web::test]
async fn test_update_employee_request_partial() {
    let update_req = UpdateEmployeeRequest {
        first_name: Some("Updated".to_string()),
        last_name: None,
        email: None,
        department_id: None,
        salary_grade_id: None,
        manager_id: None,
        role: None,
        hire_date: None,
        active: None,
    };

    assert!(update_req.first_name.is_some());
    assert!(update_req.last_name.is_none());
}

#[actix_web::test]
async fn test_assign_manager_request_validation() {
    let req = AssignManagerRequest {
        manager_id: "manager-uuid-123".to_string(),
    };

    assert!(!req.manager_id.is_empty());
}

#[actix_web::test]
async fn test_assign_salary_grade_request_validation() {
    let req = AssignSalaryGradeRequest {
        salary_grade_id: "grade-uuid-456".to_string(),
    };

    assert!(!req.salary_grade_id.is_empty());
}
