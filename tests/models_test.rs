// Tests for models module

#[cfg(test)]
mod tests {
    use backend::models::*;
    use serde_json;

    #[test]
    fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "ok".to_string(),
            message: "Test message".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("ok"));
        assert!(json.contains("Test message"));
    }

    #[test]
    fn test_department_serialization() {
        let department = Department {
            id: "test-id".to_string(),
            name: "Engineering".to_string(),
            head_id: Some("head-id".to_string()),
            created_at: None,
            updated_at: None,
        };

        let json = serde_json::to_string(&department).unwrap();
        assert!(json.contains("Engineering"));
        assert!(json.contains("head-id"));
    }

    #[test]
    fn test_department_without_head() {
        let department = Department {
            id: "test-id".to_string(),
            name: "Engineering".to_string(),
            head_id: None,
            created_at: None,
            updated_at: None,
        };

        assert_eq!(department.head_id, None);
        assert_eq!(department.name, "Engineering");
    }

    #[test]
    fn test_create_department_request_deserialization() {
        let json = r#"{"name":"HR","head_id":"some-id"}"#;
        let request: CreateDepartmentRequest = serde_json::from_str(json).unwrap();
        
        assert_eq!(request.name, "HR");
        assert_eq!(request.head_id, Some("some-id".to_string()));
    }

    #[test]
    fn test_salary_grade_model() {
        let grade = SalaryGrade {
            id: "grade-1".to_string(),
            code: "E1".to_string(),
            base_salary: 45000.0,
            description: Some("Entry level".to_string()),
            created_at: None,
        };

        assert_eq!(grade.code, "E1");
        assert_eq!(grade.base_salary, 45000.0);
    }

    #[test]
    fn test_create_salary_grade_request() {
        let json = r#"{"code":"E2","base_salary":55000.0,"description":"Junior"}"#;
        let request: CreateSalaryGradeRequest = serde_json::from_str(json).unwrap();
        
        assert_eq!(request.code, "E2");
        assert_eq!(request.base_salary, 55000.0);
        assert_eq!(request.description, Some("Junior".to_string()));
    }

    #[test]
    fn test_employee_model() {
        let employee = Employee {
            id: "emp-1".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            department_id: Some("dept-1".to_string()),
            salary_grade_id: Some("grade-1".to_string()),
            manager_id: None,
            role: "Employee".to_string(),
            hire_date: Some("2023-01-15".to_string()),
            active: true,
            deleted_at: None,
            created_at: None,
            updated_at: None,
        };

        assert_eq!(employee.first_name, "John");
        assert_eq!(employee.last_name, "Doe");
        assert!(employee.active);
        assert_eq!(employee.role, "Employee");
    }

    #[test]
    fn test_create_employee_request_deserialization() {
        let json = r#"{
            "first_name":"Jane",
            "last_name":"Smith",
            "email":"jane.smith@example.com",
            "department_id":"dept-1",
            "salary_grade_id":"grade-1",
            "manager_id":"mgr-1",
            "role":"DepartmentHead",
            "hire_date":"2023-01-01"
        }"#;
        
        let request: CreateEmployeeRequest = serde_json::from_str(json).unwrap();
        
        assert_eq!(request.first_name, "Jane");
        assert_eq!(request.last_name, "Smith");
        assert_eq!(request.role, Some("DepartmentHead".to_string()));
    }

    #[test]
    fn test_update_employee_request_partial() {
        let json = r#"{"first_name":"UpdatedName"}"#;
        let request: UpdateEmployeeRequest = serde_json::from_str(json).unwrap();
        
        assert_eq!(request.first_name, Some("UpdatedName".to_string()));
        assert_eq!(request.last_name, None);
        assert_eq!(request.email, None);
    }

    #[test]
    fn test_assign_manager_request() {
        let json = r#"{"manager_id":"mgr-123"}"#;
        let request: AssignManagerRequest = serde_json::from_str(json).unwrap();
        
        assert_eq!(request.manager_id, "mgr-123");
    }

    #[test]
    fn test_assign_salary_grade_request() {
        let json = r#"{"salary_grade_id":"grade-456"}"#;
        let request: AssignSalaryGradeRequest = serde_json::from_str(json).unwrap();
        
        assert_eq!(request.salary_grade_id, "grade-456");
    }

    #[test]
    fn test_employee_roles() {
        let roles = vec!["Admin", "DepartmentHead", "DeputyHead", "Employee"];
        
        for role in roles {
            let employee = Employee {
                id: "test".to_string(),
                first_name: "Test".to_string(),
                last_name: "User".to_string(),
                email: "test@example.com".to_string(),
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
            
            assert!(vec!["Admin", "DepartmentHead", "DeputyHead", "Employee"].contains(&employee.role.as_str()));
        }
    }
}
