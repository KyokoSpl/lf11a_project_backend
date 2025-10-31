// Tests for basic handler module

#[cfg(test)]
mod tests {
    use backend::models::*;

    #[test]
    fn test_health_response_structure() {
        let response = HealthResponse {
            status: "ok".to_string(),
            message: "Server is running".to_string(),
        };

        assert_eq!(response.status, "ok");
        assert_eq!(response.message, "Server is running");
    }

    #[test]
    fn test_create_user_request_validation() {
        let user_request = CreateUserRequest {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };

        assert!(!user_request.name.is_empty());
        assert!(!user_request.email.is_empty());
        assert!(user_request.email.contains('@'));
    }

    #[test]
    fn test_user_model() {
        let user = User {
            id: 1,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        };

        assert_eq!(user.id, 1);
        assert_eq!(user.name, "John Doe");
        assert!(user.email.contains("@"));
    }

    #[test]
    fn test_user_json_serialization() {
        let user = User {
            id: 42,
            name: "Jane Doe".to_string(),
            email: "jane@example.com".to_string(),
        };

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("42"));
        assert!(json.contains("Jane Doe"));
        assert!(json.contains("jane@example.com"));
    }

    #[test]
    fn test_create_user_request_deserialization() {
        let json = r#"{"name":"Test User","email":"test@example.com"}"#;
        let request: CreateUserRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.name, "Test User");
        assert_eq!(request.email, "test@example.com");
    }
}
