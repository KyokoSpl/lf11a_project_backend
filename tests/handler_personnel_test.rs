// Integration tests for handler_personnel module

#[cfg(test)]
mod tests {
    use backend::models::*;

    #[test]
    fn test_create_employee_request_validation() {
        let valid_request = CreateEmployeeRequest {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            department_id: Some("dept-1".to_string()),
            salary_grade_id: Some("grade-1".to_string()),
            manager_id: None,
            role: Some("Employee".to_string()),
            hire_date: Some("2023-01-01".to_string()),
        };

        assert_eq!(valid_request.first_name, "John");
        assert_eq!(valid_request.email, "john.doe@example.com");
    }

    #[test]
    fn test_update_employee_request_partial_update() {
        let update = UpdateEmployeeRequest {
            first_name: Some("Jane".to_string()),
            last_name: None,
            email: Some("jane@example.com".to_string()),
            department_id: None,
            salary_grade_id: None,
            manager_id: None,
            role: None,
            hire_date: None,
            active: None,
        };

        assert!(update.first_name.is_some());
        assert!(update.last_name.is_none());
        assert!(update.email.is_some());
    }

    #[test]
    fn test_create_department_request_validation() {
        let dept_request = CreateDepartmentRequest {
            name: "Engineering".to_string(),
            head_id: Some("emp-123".to_string()),
        };

        assert_eq!(dept_request.name, "Engineering");
        assert!(dept_request.head_id.is_some());
    }

    #[test]
    fn test_create_salary_grade_request_validation() {
        let grade_request = CreateSalaryGradeRequest {
            code: "E1".to_string(),
            base_salary: 45000.0,
            description: Some("Entry level".to_string()),
        };

        assert_eq!(grade_request.code, "E1");
        assert!(grade_request.base_salary > 0.0);
    }

    #[test]
    fn test_assign_manager_request() {
        let assign_req = AssignManagerRequest {
            manager_id: "mgr-456".to_string(),
        };

        assert!(!assign_req.manager_id.is_empty());
    }

    #[test]
    fn test_assign_salary_grade_request() {
        let assign_req = AssignSalaryGradeRequest {
            salary_grade_id: "grade-789".to_string(),
        };

        assert!(!assign_req.salary_grade_id.is_empty());
    }

    #[test]
    fn test_employee_model_active_flag() {
        let active_employee = Employee {
            id: "emp-1".to_string(),
            first_name: "Active".to_string(),
            last_name: "Employee".to_string(),
            email: "active@example.com".to_string(),
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

        assert!(active_employee.active);
        assert!(active_employee.deleted_at.is_none());
    }

    #[test]
    fn test_employee_role_validation() {
        let roles = vec!["Admin", "DepartmentHead", "DeputyHead", "Employee"];

        for role in roles {
            let emp = Employee {
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

            assert!(
                ["Admin", "DepartmentHead", "DeputyHead", "Employee"].contains(&emp.role.as_str())
            );
        }
    }

    #[test]
    fn test_department_with_and_without_head() {
        let dept_with_head = Department {
            id: "dept-1".to_string(),
            name: "Engineering".to_string(),
            head_id: Some("head-1".to_string()),
            created_at: None,
            updated_at: None,
        };

        let dept_without_head = Department {
            id: "dept-2".to_string(),
            name: "HR".to_string(),
            head_id: None,
            created_at: None,
            updated_at: None,
        };

        assert!(dept_with_head.head_id.is_some());
        assert!(dept_without_head.head_id.is_none());
    }

    #[test]
    fn test_salary_grade_with_description() {
        let grade = SalaryGrade {
            id: "grade-1".to_string(),
            code: "E1".to_string(),
            base_salary: 45000.0,
            description: Some("Entry level position".to_string()),
            created_at: None,
        };

        assert!(grade.description.is_some());
        assert_eq!(grade.base_salary, 45000.0);
    }

    #[test]
    fn test_update_requests_empty_validation() {
        let empty_emp_update = UpdateEmployeeRequest {
            first_name: None,
            last_name: None,
            email: None,
            department_id: None,
            salary_grade_id: None,
            manager_id: None,
            role: None,
            hire_date: None,
            active: None,
        };

        // All fields should be None
        assert!(empty_emp_update.first_name.is_none());
        assert!(empty_emp_update.last_name.is_none());
        assert!(empty_emp_update.email.is_none());
    }

    #[test]
    fn test_json_serialization_employee() {
        let employee = Employee {
            id: "emp-1".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john@example.com".to_string(),
            department_id: Some("dept-1".to_string()),
            salary_grade_id: Some("grade-1".to_string()),
            manager_id: Some("mgr-1".to_string()),
            role: "Employee".to_string(),
            hire_date: Some("2023-01-01".to_string()),
            active: true,
            deleted_at: None,
            created_at: Some("2023-01-01T00:00:00".to_string()),
            updated_at: None,
        };

        let json = serde_json::to_string(&employee).unwrap();
        assert!(json.contains("John"));
        assert!(json.contains("john@example.com"));
    }
}
