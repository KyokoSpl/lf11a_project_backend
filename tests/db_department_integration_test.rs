mod common;

use backend::models::*;
use common::*;

#[test]
fn test_get_departments_with_database() {
    let pool = setup_test_db().unwrap();

    let initial_count = count_departments(&pool).unwrap();

    // Create test departments
    let dept1_id = create_test_department(&pool, "IT Department").unwrap();
    let dept2_id = create_test_department(&pool, "HR Department").unwrap();

    let final_count = count_departments(&pool).unwrap();
    assert_eq!(
        final_count,
        initial_count + 2,
        "Department count should increase by 2"
    );

    // Cleanup
    delete_test_department(&pool, &dept1_id).ok();
    delete_test_department(&pool, &dept2_id).ok();
}

#[test]
fn test_create_department_in_database() {
    let pool = setup_test_db().unwrap();

    let dept_id = create_test_department(&pool, "Finance Department").unwrap();

    assert!(
        department_exists(&pool, &dept_id).unwrap(),
        "Department should exist in database"
    );

    // Cleanup
    delete_test_department(&pool, &dept_id).ok();
}

#[test]
fn test_department_with_employees() {
    let pool = setup_test_db().unwrap();

    // Create a department
    let dept_id = create_test_department(&pool, "Engineering").unwrap();

    // Create multiple employees in this department
    let emp1_id = create_test_employee(
        &pool,
        "Alice",
        "Smith",
        &format!("alice_{}@test.com", uuid::Uuid::new_v4()),
        Some(&dept_id),
        None,
    )
    .unwrap();

    let emp2_id = create_test_employee(
        &pool,
        "Bob",
        "Johnson",
        &format!("bob_{}@test.com", uuid::Uuid::new_v4()),
        Some(&dept_id),
        None,
    )
    .unwrap();

    // Verify department and employees exist
    assert!(department_exists(&pool, &dept_id).unwrap());
    assert!(employee_exists(&pool, &emp1_id).unwrap());
    assert!(employee_exists(&pool, &emp2_id).unwrap());

    // Cleanup
    delete_test_employee(&pool, &emp1_id).ok();
    delete_test_employee(&pool, &emp2_id).ok();
    delete_test_department(&pool, &dept_id).ok();
}

#[test]
fn test_department_without_head() {
    let pool = setup_test_db().unwrap();

    let dept_id = create_test_department(&pool, "Marketing").unwrap();

    assert!(
        department_exists(&pool, &dept_id).unwrap(),
        "Department without head should exist"
    );

    // Cleanup
    delete_test_department(&pool, &dept_id).ok();
}

#[test]
fn test_multiple_departments() {
    let pool = setup_test_db().unwrap();

    let initial_count = count_departments(&pool).unwrap();

    // Create multiple departments
    let dept_ids: Vec<String> = (0..5)
        .map(|i| create_test_department(&pool, &format!("Dept_{}", i)).unwrap())
        .collect();

    let final_count = count_departments(&pool).unwrap();
    assert_eq!(
        final_count,
        initial_count + 5,
        "Should have 5 more departments"
    );

    // Verify all exist
    for dept_id in &dept_ids {
        assert!(
            department_exists(&pool, dept_id).unwrap(),
            "Department {} should exist",
            dept_id
        );
    }

    // Cleanup
    for dept_id in dept_ids {
        delete_test_department(&pool, &dept_id).ok();
    }
}

#[test]
fn test_create_department_request_validation() {
    // Verify the CreateDepartmentRequest structure
    let request = CreateDepartmentRequest {
        name: "Test Department".to_string(),
        head_id: None,
    };

    assert_eq!(request.name, "Test Department");
    assert!(request.head_id.is_none());
}

#[test]
fn test_update_department_request_structure() {
    // Verify the UpdateDepartmentRequest structure
    let request = UpdateDepartmentRequest {
        name: Some("Updated Department".to_string()),
        head_id: Some("some-head-id".to_string()),
    };

    assert!(request.name.is_some());
    assert!(request.head_id.is_some());
}

#[test]
fn test_department_crud_operations() {
    let pool = setup_test_db().unwrap();

    let initial_count = count_departments(&pool).unwrap();

    // CREATE
    let dept_id = create_test_department(&pool, "Operations").unwrap();
    assert!(
        department_exists(&pool, &dept_id).unwrap(),
        "Department should be created"
    );

    // READ (verify count increased)
    let after_create_count = count_departments(&pool).unwrap();
    assert_eq!(
        after_create_count,
        initial_count + 1,
        "Count should increase by 1"
    );

    // DELETE
    delete_test_department(&pool, &dept_id).ok();
    assert!(
        !department_exists(&pool, &dept_id).unwrap(),
        "Department should be deleted"
    );

    // Verify count returned to initial
    let after_delete_count = count_departments(&pool).unwrap();
    assert_eq!(
        after_delete_count, initial_count,
        "Count should return to initial"
    );
}

#[test]
fn test_department_count_tracking() {
    let pool = setup_test_db().unwrap();

    let initial_count = count_departments(&pool).unwrap();

    // Add departments
    let dept1_id = create_test_department(&pool, "Logistics").unwrap();
    assert_eq!(count_departments(&pool).unwrap(), initial_count + 1);

    let dept2_id = create_test_department(&pool, "Legal").unwrap();
    assert_eq!(count_departments(&pool).unwrap(), initial_count + 2);

    // Remove departments
    delete_test_department(&pool, &dept1_id).ok();
    assert_eq!(count_departments(&pool).unwrap(), initial_count + 1);

    delete_test_department(&pool, &dept2_id).ok();
    assert_eq!(count_departments(&pool).unwrap(), initial_count);
}
