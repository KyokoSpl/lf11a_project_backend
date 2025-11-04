// Tests for the handlers module structure and exports

use backend::models::*;

#[test]
fn test_employee_request_models() {
    let create_req = CreateEmployeeRequest {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john@example.com".to_string(),
        department_id: Some("dept-1".to_string()),
        salary_grade_id: Some("grade-1".to_string()),
        manager_id: None,
        role: Some("Employee".to_string()),
        hire_date: Some("2024-01-01".to_string()),
    };

    assert_eq!(create_req.first_name, "John");
    assert_eq!(create_req.last_name, "Doe");
    assert!(create_req.email.contains("@"));
}

#[test]
fn test_update_employee_request() {
    let update_req = UpdateEmployeeRequest {
        first_name: Some("Jane".to_string()),
        last_name: Some("Smith".to_string()),
        email: None,
        department_id: None,
        salary_grade_id: None,
        manager_id: None,
        role: None,
        hire_date: None,
        active: Some(true),
    };

    assert!(update_req.first_name.is_some());
    assert!(update_req.last_name.is_some());
    assert!(update_req.email.is_none());
}

#[test]
fn test_assign_manager_request() {
    let req = AssignManagerRequest {
        manager_id: "mgr-123".to_string(),
    };

    assert_eq!(req.manager_id, "mgr-123");
}

#[test]
fn test_assign_salary_grade_request() {
    let req = AssignSalaryGradeRequest {
        salary_grade_id: "grade-456".to_string(),
    };

    assert_eq!(req.salary_grade_id, "grade-456");
}

#[test]
fn test_department_request_models() {
    let create_req = CreateDepartmentRequest {
        name: "Engineering".to_string(),
        head_id: Some("head-1".to_string()),
    };

    assert_eq!(create_req.name, "Engineering");
    assert!(create_req.head_id.is_some());
}

#[test]
fn test_update_department_request() {
    let update_req = UpdateDepartmentRequest {
        name: Some("Sales".to_string()),
        head_id: None,
    };

    assert_eq!(update_req.name.unwrap(), "Sales");
    assert!(update_req.head_id.is_none());
}

#[test]
fn test_salary_grade_request_models() {
    let create_req = CreateSalaryGradeRequest {
        code: "E1".to_string(),
        base_salary: 50000.0,
        description: Some("Entry level".to_string()),
    };

    assert_eq!(create_req.code, "E1");
    assert_eq!(create_req.base_salary, 50000.0);
}

#[test]
fn test_update_salary_grade_request() {
    let update_req = UpdateSalaryGradeRequest {
        code: None,
        base_salary: Some(55000.0),
        description: Some("Updated".to_string()),
    };

    assert!(update_req.code.is_none());
    assert_eq!(update_req.base_salary.unwrap(), 55000.0);
}

#[test]
fn test_employee_model_fields() {
    let employee = Employee {
        id: "emp-1".to_string(),
        first_name: "Alice".to_string(),
        last_name: "Johnson".to_string(),
        email: "alice@example.com".to_string(),
        department_id: Some("dept-1".to_string()),
        salary_grade_id: Some("grade-1".to_string()),
        manager_id: Some("mgr-1".to_string()),
        role: "Employee".to_string(),
        hire_date: Some("2024-01-01".to_string()),
        active: true,
        deleted_at: None,
        created_at: Some("2024-01-01".to_string()),
        updated_at: Some("2024-01-01".to_string()),
    };

    assert_eq!(employee.first_name, "Alice");
    assert_eq!(employee.role, "Employee");
    assert!(employee.active);
}

#[test]
fn test_department_model_fields() {
    let department = Department {
        id: "dept-1".to_string(),
        name: "Marketing".to_string(),
        head_id: Some("head-1".to_string()),
        created_at: Some("2024-01-01".to_string()),
        updated_at: Some("2024-01-01".to_string()),
    };

    assert_eq!(department.name, "Marketing");
    assert!(department.head_id.is_some());
}

#[test]
fn test_salary_grade_model_fields() {
    let grade = SalaryGrade {
        id: "grade-1".to_string(),
        code: "M1".to_string(),
        base_salary: 75000.0,
        description: Some("Mid-level".to_string()),
        created_at: Some("2024-01-01".to_string()),
    };

    assert_eq!(grade.code, "M1");
    assert_eq!(grade.base_salary, 75000.0);
}

#[test]
fn test_employee_json_round_trip() {
    let employee = Employee {
        id: "emp-1".to_string(),
        first_name: "Bob".to_string(),
        last_name: "Wilson".to_string(),
        email: "bob@example.com".to_string(),
        department_id: None,
        salary_grade_id: None,
        manager_id: None,
        role: "Admin".to_string(),
        hire_date: None,
        active: true,
        deleted_at: None,
        created_at: None,
        updated_at: None,
    };

    let json = serde_json::to_string(&employee).unwrap();
    let deserialized: Employee = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.first_name, "Bob");
    assert_eq!(deserialized.last_name, "Wilson");
    assert_eq!(deserialized.role, "Admin");
}

#[test]
fn test_department_json_round_trip() {
    let department = Department {
        id: "dept-2".to_string(),
        name: "Finance".to_string(),
        head_id: None,
        created_at: None,
        updated_at: None,
    };

    let json = serde_json::to_string(&department).unwrap();
    let deserialized: Department = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.name, "Finance");
}

#[test]
fn test_salary_grade_json_round_trip() {
    let grade = SalaryGrade {
        id: "grade-2".to_string(),
        code: "S1".to_string(),
        base_salary: 100000.0,
        description: None,
        created_at: None,
    };

    let json = serde_json::to_string(&grade).unwrap();
    let deserialized: SalaryGrade = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.code, "S1");
    assert_eq!(deserialized.base_salary, 100000.0);
}

#[test]
fn test_all_employee_roles() {
    let roles = vec!["Admin", "DepartmentHead", "DeputyHead", "Employee"];

    for role in &roles {
        let employee = Employee {
            id: format!("emp-{}", role),
            first_name: role.to_string(),
            last_name: "Test".to_string(),
            email: format!("{}@example.com", role.to_lowercase()),
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

        assert_eq!(&employee.role, role);
    }
}

#[test]
fn test_employee_with_all_relationships() {
    let employee = Employee {
        id: "emp-full".to_string(),
        first_name: "Complete".to_string(),
        last_name: "Employee".to_string(),
        email: "complete@example.com".to_string(),
        department_id: Some("dept-1".to_string()),
        salary_grade_id: Some("grade-1".to_string()),
        manager_id: Some("mgr-1".to_string()),
        role: "Employee".to_string(),
        hire_date: Some("2024-01-01".to_string()),
        active: true,
        deleted_at: None,
        created_at: Some("2024-01-01".to_string()),
        updated_at: Some("2024-01-01".to_string()),
    };

    assert!(employee.department_id.is_some());
    assert!(employee.salary_grade_id.is_some());
    assert!(employee.manager_id.is_some());
    assert!(employee.hire_date.is_some());
}

#[test]
fn test_employee_without_relationships() {
    let employee = Employee {
        id: "emp-min".to_string(),
        first_name: "Minimal".to_string(),
        last_name: "Employee".to_string(),
        email: "minimal@example.com".to_string(),
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

    assert!(employee.department_id.is_none());
    assert!(employee.salary_grade_id.is_none());
    assert!(employee.manager_id.is_none());
}

#[test]
fn test_multiple_salary_grades() {
    let grades = vec![
        ("E1", 40000.0),
        ("E2", 50000.0),
        ("M1", 70000.0),
        ("M2", 90000.0),
        ("S1", 110000.0),
    ];

    for (code, salary) in grades {
        let grade = SalaryGrade {
            id: format!("grade-{}", code),
            code: code.to_string(),
            base_salary: salary,
            description: Some(format!("{} level", code)),
            created_at: None,
        };

        assert_eq!(grade.code, code);
        assert_eq!(grade.base_salary, salary);
    }
}

#[test]
fn test_multiple_departments() {
    let departments = vec!["Engineering", "Sales", "Marketing", "HR", "Finance"];

    for dept_name in departments {
        let dept = Department {
            id: format!("dept-{}", dept_name.to_lowercase()),
            name: dept_name.to_string(),
            head_id: None,
            created_at: None,
            updated_at: None,
        };

        assert_eq!(dept.name, dept_name);
    }
}
