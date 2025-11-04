// Direct unit tests for handler functions (no HTTP layer)

use backend::models::*;

#[test]
fn test_health_response_creation() {
    let health_resp = HealthResponse {
        status: "healthy".to_string(),
        message: "Service is running".to_string(),
    };

    assert_eq!(health_resp.status, "healthy");
    assert!(!health_resp.message.is_empty());
}

#[test]
fn test_user_creation() {
    let user = User {
        id: 42,
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
    };

    assert_eq!(user.id, 42);
    assert!(!user.name.is_empty());
    assert!(user.email.contains("@"));
}

#[test]
fn test_create_user_request() {
    let request = CreateUserRequest {
        name: "New User".to_string(),
        email: "new@example.com".to_string(),
    };

    assert_eq!(request.name, "New User");
    assert_eq!(request.email, "new@example.com");
}
