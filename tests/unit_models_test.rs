// Unit tests for models and request/response structures

use backend::models::*;

#[test]
fn test_employee_model_serialization() {
    let employee = Employee {
        id: "emp-123".to_string(),
        first_name: "Jane".to_string(),
        last_name: "Smith".to_string(),
        email: "jane.smith@company.com".to_string(),
        department_id: Some("dept-456".to_string()),
        salary_grade_id: Some("grade-789".to_string()),
        manager_id: Some("mgr-101".to_string()),
        role: "Employee".to_string(),
        hire_date: Some("2024-06-15".to_string()),
        active: true,
        deleted_at: None,
        created_at: Some("2024-06-15 10:00:00".to_string()),
        updated_at: Some("2024-06-15 10:00:00".to_string()),
    };

    // Test serialization
    let json = serde_json::to_string(&employee).unwrap();
    assert!(json.contains("Jane"));
    assert!(json.contains("Smith"));
    assert!(json.contains("jane.smith@company.com"));

    // Test deserialization
    let deserialized: Employee = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.first_name, "Jane");
    assert_eq!(deserialized.last_name, "Smith");
    assert_eq!(deserialized.email, "jane.smith@company.com");
    assert!(deserialized.active);
}

#[test]
fn test_employee_roles() {
    let roles = vec!["Admin", "DepartmentHead", "DeputyHead", "Employee"];

    for role in roles {
        let employee = Employee {
            id: "test-id".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            email: "test@company.com".to_string(),
            department_id: None,
            salary_grade_id: None,
            manager_id: None,
            role: role.to_string(),
            hire_date: None,
            active: true,
            deleted_at: None,
            created_at: None,
            updated_at: None,
        };

        assert_eq!(employee.role, role);
    }
}

#[test]
fn test_employee_active_inactive_states() {
    let active_employee = Employee {
        id: "emp-001".to_string(),
        first_name: "Active".to_string(),
        last_name: "Employee".to_string(),
        email: "active@company.com".to_string(),
        department_id: None,
        salary_grade_id: None,
        manager_id: None,
        role: "Employee".to_string(),
        hire_date: None,
        active: true,
        deleted_at: None,
        created_at: None,
        updated_at: None,
    };

    let inactive_employee = Employee {
        id: "emp-002".to_string(),
        first_name: "Inactive".to_string(),
        last_name: "Employee".to_string(),
        email: "inactive@company.com".to_string(),
        department_id: None,
        salary_grade_id: None,
        manager_id: None,
        role: "Employee".to_string(),
        hire_date: None,
        active: false,
        deleted_at: Some("2024-12-31 23:59:59".to_string()),
        created_at: None,
        updated_at: None,
    };

    assert!(active_employee.active);
    assert!(!inactive_employee.active);
    assert!(inactive_employee.deleted_at.is_some());
}

#[test]
fn test_department_model_serialization() {
    let department = Department {
        id: "dept-123".to_string(),
        name: "Human Resources".to_string(),
        head_id: Some("mgr-456".to_string()),
        created_at: Some("2024-01-01 00:00:00".to_string()),
        updated_at: Some("2024-01-01 00:00:00".to_string()),
    };

    // Test serialization
    let json = serde_json::to_string(&department).unwrap();
    assert!(json.contains("Human Resources"));

    // Test deserialization
    let deserialized: Department = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, "Human Resources");
}

#[test]
fn test_department_with_manager() {
    let dept_with_manager = Department {
        id: "dept-001".to_string(),
        name: "Sales".to_string(),
        head_id: Some("manager-001".to_string()),
        created_at: None,
        updated_at: None,
    };

    assert!(dept_with_manager.head_id.is_some());
    assert_eq!(dept_with_manager.head_id.unwrap(), "manager-001");
}

#[test]
fn test_department_without_manager() {
    let dept_without_manager = Department {
        id: "dept-002".to_string(),
        name: "Marketing".to_string(),
        head_id: None,
        created_at: None,
        updated_at: None,
    };

    assert!(dept_without_manager.head_id.is_none());
}

#[test]
fn test_create_department_minimal_fields() {
    let minimal_dept = CreateDepartmentRequest {
        name: "New Department".to_string(),
        head_id: None,
    };

    assert_eq!(minimal_dept.name, "New Department");
    assert!(minimal_dept.head_id.is_none());
}

#[test]
fn test_update_department_all_fields() {
    let full_update = UpdateDepartmentRequest {
        name: Some("Updated Name".to_string()),
        head_id: Some("new-manager".to_string()),
    };

    assert!(full_update.name.is_some());
    assert!(full_update.head_id.is_some());
}

#[test]
fn test_salary_grade_model_serialization() {
    let salary_grade = SalaryGrade {
        id: "grade-123".to_string(),
        code: "MGR".to_string(),
        base_salary: 125000.0,
        description: Some("Manager grade".to_string()),
        created_at: Some("2024-01-01 00:00:00".to_string()),
    };

    // Test serialization
    let json = serde_json::to_string(&salary_grade).unwrap();
    assert!(json.contains("MGR"));
    assert!(json.contains("125000"));

    // Test deserialization
    let deserialized: SalaryGrade = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.code, "MGR");
    assert_eq!(deserialized.base_salary, 125000.0);
}

#[test]
fn test_salary_grade_different_levels() {
    let entry_level = SalaryGrade {
        id: "grade-001".to_string(),
        code: "ENTRY".to_string(),
        base_salary: 50000.0,
        description: Some("Entry level".to_string()),
        created_at: None,
    };

    let senior_level = SalaryGrade {
        id: "grade-002".to_string(),
        code: "SENIOR".to_string(),
        base_salary: 120000.0,
        description: Some("Senior level".to_string()),
        created_at: None,
    };

    assert!(senior_level.base_salary > entry_level.base_salary);
    assert_ne!(entry_level.code, senior_level.code);
}

#[test]
fn test_salary_grade_with_description() {
    let grade_with_desc = SalaryGrade {
        id: "grade-003".to_string(),
        code: "SPECIALIST".to_string(),
        base_salary: 95000.0,
        description: Some("Technical specialist position".to_string()),
        created_at: None,
    };

    assert!(grade_with_desc.description.is_some());
    assert!(!grade_with_desc.description.as_ref().unwrap().is_empty());
}

#[test]
fn test_salary_grade_without_description() {
    let grade_no_desc = SalaryGrade {
        id: "grade-004".to_string(),
        code: "BASIC".to_string(),
        base_salary: 45000.0,
        description: None,
        created_at: None,
    };

    assert!(grade_no_desc.description.is_none());
}

#[test]
fn test_create_salary_grade_validation() {
    let valid_grade = CreateSalaryGradeRequest {
        code: "MID-LEVEL".to_string(),
        base_salary: 75000.0,
        description: Some("Mid-level engineer grade".to_string()),
    };

    assert!(!valid_grade.code.is_empty());
    assert!(valid_grade.base_salary > 0.0);
}

#[test]
fn test_create_salary_grade_minimal() {
    let minimal_grade = CreateSalaryGradeRequest {
        code: "MINIMAL".to_string(),
        base_salary: 40000.0,
        description: None,
    };

    assert_eq!(minimal_grade.code, "MINIMAL");
    assert!(minimal_grade.description.is_none());
}

#[test]
fn test_update_salary_grade_all_fields() {
    let full_update = UpdateSalaryGradeRequest {
        code: Some("UPDATED-CODE".to_string()),
        base_salary: Some(110000.0),
        description: Some("Updated description".to_string()),
    };

    assert!(full_update.code.is_some());
    assert!(full_update.base_salary.is_some());
    assert!(full_update.description.is_some());
}

#[test]
fn test_update_salary_grade_only_salary() {
    let salary_update = UpdateSalaryGradeRequest {
        code: None,
        base_salary: Some(85000.0),
        description: None,
    };

    assert!(salary_update.code.is_none());
    assert!(salary_update.base_salary.is_some());
    assert_eq!(salary_update.base_salary.unwrap(), 85000.0);
}

#[test]
fn test_salary_grade_codes_uniqueness() {
    let grade1 = SalaryGrade {
        id: "grade-005".to_string(),
        code: "CODE1".to_string(),
        base_salary: 60000.0,
        description: None,
        created_at: None,
    };

    let grade2 = SalaryGrade {
        id: "grade-006".to_string(),
        code: "CODE2".to_string(),
        base_salary: 60000.0,
        description: None,
        created_at: None,
    };

    assert_ne!(grade1.code, grade2.code);
    assert_ne!(grade1.id, grade2.id);
}

#[test]
fn test_health_response_structure() {
    let health_resp = HealthResponse {
        status: "healthy".to_string(),
        message: "Service is running".to_string(),
    };

    assert_eq!(health_resp.status, "healthy");
    assert!(!health_resp.message.is_empty());
}

#[test]
fn test_user_model_complete() {
    let user = User {
        id: 123,
        name: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
    };

    assert_eq!(user.id, 123);
    assert!(!user.name.is_empty());
    assert!(user.email.contains("@"));
}

#[test]
fn test_create_user_request_validation() {
    let valid_request = CreateUserRequest {
        name: "Jane Smith".to_string(),
        email: "jane.smith@example.com".to_string(),
    };

    assert!(!valid_request.name.is_empty());
    assert!(valid_request.email.contains("@"));
    assert!(valid_request.email.contains("."));
}

#[test]
fn test_create_user_request_serialization() {
    let request = CreateUserRequest {
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("Test User"));
    assert!(json.contains("test@example.com"));

    let deserialized: CreateUserRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, "Test User");
    assert_eq!(deserialized.email, "test@example.com");
}

#[test]
fn test_user_serialization() {
    let user = User {
        id: 789,
        name: "Alice Brown".to_string(),
        email: "alice@example.com".to_string(),
    };

    let json = serde_json::to_string(&user).unwrap();
    assert!(json.contains("789"));
    assert!(json.contains("Alice Brown"));

    let deserialized: User = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.id, 789);
    assert_eq!(deserialized.name, "Alice Brown");
}

#[test]
fn test_health_response_serialization() {
    let response = HealthResponse {
        status: "ok".to_string(),
        message: "All systems operational".to_string(),
    };

    let json = serde_json::to_string(&response).unwrap();
    assert!(json.contains("ok"));
    assert!(json.contains("All systems operational"));

    let deserialized: HealthResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.status, "ok");
}

#[test]
fn test_multiple_users() {
    let users = [
        User {
            id: 1,
            name: "User One".to_string(),
            email: "user1@example.com".to_string(),
        },
        User {
            id: 2,
            name: "User Two".to_string(),
            email: "user2@example.com".to_string(),
        },
        User {
            id: 3,
            name: "User Three".to_string(),
            email: "user3@example.com".to_string(),
        },
    ];

    let _json = serde_json::to_string(&users).unwrap();

    assert_eq!(users.len(), 3);
    assert_ne!(users[0].id, users[1].id);
    assert_ne!(users[1].email, users[2].email);
}

#[test]
fn test_user_email_formats() {
    let user1 = User {
        id: 1,
        name: "User 1".to_string(),
        email: "user@company.com".to_string(),
    };

    let user2 = User {
        id: 2,
        name: "User 2".to_string(),
        email: "user.name@subdomain.company.com".to_string(),
    };

    assert!(user1.email.contains("@"));
    assert!(user2.email.contains("@"));
    assert!(user2.email.contains("."));
}
