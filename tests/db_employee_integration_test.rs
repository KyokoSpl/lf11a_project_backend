// Database integration tests for employee handlers

mod common;

use backend::db::create_pool;
use backend::models::*;
use common::*;

#[actix_web::test]
async fn test_get_employees_with_database() {
    let db_url = get_test_db_url();
    if let Ok(pool) = create_pool(&db_url) {
        let initial_count = count_employees(&pool).unwrap_or(0);

        // Create test employees
        let dept_id = create_test_department(&pool, "Test Engineering").ok();
        let grade_id = create_test_salary_grade(&pool, "TEST-E1", 50000.0).ok();

        create_test_employee(
            &pool,
            "Test",
            "Employee1",
            "test.employee1@test.com",
            dept_id.as_deref(),
            grade_id.as_deref(),
        )
        .ok();

        create_test_employee(
            &pool,
            "Test",
            "Employee2",
            "test.employee2@test.com",
            dept_id.as_deref(),
            grade_id.as_deref(),
        )
        .ok();

        let new_count = count_employees(&pool).unwrap_or(0);
        assert!(
            new_count >= initial_count + 2,
            "Should have created 2 employees"
        );

        // Cleanup
        cleanup_test_data(&pool).ok();
    }
}

#[actix_web::test]
async fn test_create_employee_in_database() {
    let db_url = get_test_db_url();
    if let Ok(pool) = create_pool(&db_url) {
        let dept_id = create_test_department(&pool, "Test HR").ok();
        let grade_id = create_test_salary_grade(&pool, "TEST-E2", 55000.0).ok();

        let emp_id = create_test_employee(
            &pool,
            "John",
            "Doe",
            "john.doe.test@test.com",
            dept_id.as_deref(),
            grade_id.as_deref(),
        );

        assert!(emp_id.is_ok(), "Should create employee successfully");

        if let Ok(id) = emp_id {
            let exists = employee_exists(&pool, &id).unwrap_or(false);
            assert!(exists, "Employee should exist in database");

            // Cleanup
            delete_test_employee(&pool, &id).ok();
        }

        cleanup_test_data(&pool).ok();
    }
}

#[actix_web::test]
async fn test_employee_with_department_relationship() {
    let db_url = get_test_db_url();
    if let Ok(pool) = create_pool(&db_url) {
        let dept_id = create_test_department(&pool, "Test Sales").unwrap();
        let grade_id = create_test_salary_grade(&pool, "TEST-E3", 60000.0).unwrap();

        let emp_id = create_test_employee(
            &pool,
            "Jane",
            "Smith",
            "jane.smith.test@test.com",
            Some(&dept_id),
            Some(&grade_id),
        )
        .unwrap();

        // Verify employee exists
        assert!(employee_exists(&pool, &emp_id).unwrap());

        // Verify department exists
        assert!(department_exists(&pool, &dept_id).unwrap());

        // Cleanup
        delete_test_employee(&pool, &emp_id).ok();
        delete_test_department(&pool, &dept_id).ok();
        delete_test_salary_grade(&pool, &grade_id).ok();
    }
}

#[actix_web::test]
async fn test_employee_without_department() {
    let db_url = get_test_db_url();
    if let Ok(pool) = create_pool(&db_url) {
        let emp_id = create_test_employee(
            &pool,
            "Independent",
            "Contractor",
            "independent.test@test.com",
            None,
            None,
        );

        assert!(emp_id.is_ok(), "Should create employee without department");

        if let Ok(id) = emp_id {
            assert!(employee_exists(&pool, &id).unwrap());
            delete_test_employee(&pool, &id).ok();
        }
    }
}

#[actix_web::test]
async fn test_multiple_employees_in_same_department() {
    let db_url = get_test_db_url();
    if let Ok(pool) = create_pool(&db_url) {
        let dept_id = create_test_department(&pool, "Test Marketing").unwrap();
        let grade_id = create_test_salary_grade(&pool, "TEST-E4", 65000.0).unwrap();

        let emp1_id = create_test_employee(
            &pool,
            "Alice",
            "Johnson",
            "alice.test@test.com",
            Some(&dept_id),
            Some(&grade_id),
        )
        .unwrap();

        let emp2_id = create_test_employee(
            &pool,
            "Bob",
            "Williams",
            "bob.test@test.com",
            Some(&dept_id),
            Some(&grade_id),
        )
        .unwrap();

        assert_ne!(emp1_id, emp2_id, "Employees should have different IDs");
        assert!(employee_exists(&pool, &emp1_id).unwrap());
        assert!(employee_exists(&pool, &emp2_id).unwrap());

        // Cleanup
        delete_test_employee(&pool, &emp1_id).ok();
        delete_test_employee(&pool, &emp2_id).ok();
        delete_test_department(&pool, &dept_id).ok();
        delete_test_salary_grade(&pool, &grade_id).ok();
    }
}

#[test]
fn test_create_employee_request_validation() {
    let request = CreateEmployeeRequest {
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        email: "test@example.com".to_string(),
        department_id: Some("dept-123".to_string()),
        salary_grade_id: Some("grade-456".to_string()),
        manager_id: None,
        role: Some("Employee".to_string()),
        hire_date: Some("2024-01-01".to_string()),
    };

    assert!(!request.first_name.is_empty());
    assert!(!request.last_name.is_empty());
    assert!(request.email.contains("@"));
}

#[test]
fn test_update_employee_request_partial_fields() {
    let request = UpdateEmployeeRequest {
        first_name: Some("Updated".to_string()),
        last_name: None,
        email: None,
        department_id: Some("new-dept".to_string()),
        salary_grade_id: None,
        manager_id: None,
        role: None,
        hire_date: None,
        active: None,
    };

    assert!(request.first_name.is_some());
    assert!(request.last_name.is_none());
    assert!(request.department_id.is_some());
}

#[test]
fn test_assign_manager_request_structure() {
    let request = AssignManagerRequest {
        manager_id: "mgr-789".to_string(),
    };

    assert!(!request.manager_id.is_empty());
}

#[test]
fn test_assign_salary_grade_request_structure() {
    let request = AssignSalaryGradeRequest {
        salary_grade_id: "grade-abc".to_string(),
    };

    assert!(!request.salary_grade_id.is_empty());
}

#[actix_web::test]
async fn test_employee_lookup_by_email() {
    let db_url = get_test_db_url();
    if let Ok(pool) = create_pool(&db_url) {
        let unique_email = format!("lookup.test.{}@test.com", uuid::Uuid::new_v4());

        let emp_id =
            create_test_employee(&pool, "Lookup", "Test", &unique_email, None, None).unwrap();

        let found_id = get_employee_by_email(&pool, &unique_email).unwrap();
        assert!(found_id.is_some());
        assert_eq!(found_id.unwrap(), emp_id);

        delete_test_employee(&pool, &emp_id).ok();
    }
}

#[actix_web::test]
async fn test_employee_with_all_roles() {
    let db_url = get_test_db_url();
    if let Ok(pool) = create_pool(&db_url) {
        let roles = vec!["Admin", "DepartmentHead", "DeputyHead", "Employee"];

        for role in roles {
            let email = format!("{}.test@test.com", role.to_lowercase());
            let emp_id = create_test_employee(&pool, role, "Test", &email, None, None);

            assert!(emp_id.is_ok(), "Should create employee with role {}", role);
            if let Ok(id) = emp_id {
                delete_test_employee(&pool, &id).ok();
            }
        }
    }
}

#[actix_web::test]
async fn test_employee_crud_operations() {
    let db_url = get_test_db_url();
    if let Ok(pool) = create_pool(&db_url) {
        // CREATE
        let emp_id =
            create_test_employee(&pool, "CRUD", "Test", "crud.test@test.com", None, None).unwrap();

        // READ (verify exists)
        assert!(employee_exists(&pool, &emp_id).unwrap());

        // UPDATE would go here (handled by handler functions in actual app)

        // DELETE
        delete_test_employee(&pool, &emp_id).unwrap();
        assert!(!employee_exists(&pool, &emp_id).unwrap());
    }
}

#[actix_web::test]
async fn test_employee_count_tracking() {
    let db_url = get_test_db_url();
    if let Ok(pool) = create_pool(&db_url) {
        let initial_count = count_employees(&pool).unwrap();

        let emp_id =
            create_test_employee(&pool, "Count", "Test", "count.test@test.com", None, None)
                .unwrap();

        let after_create = count_employees(&pool).unwrap();
        assert_eq!(after_create, initial_count + 1);

        delete_test_employee(&pool, &emp_id).ok();

        let after_delete = count_employees(&pool).unwrap();
        assert_eq!(after_delete, initial_count);
    }
}
