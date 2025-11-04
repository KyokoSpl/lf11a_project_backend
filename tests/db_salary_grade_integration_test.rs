mod common;

use backend::models::*;
use common::*;

#[test]
fn test_get_salary_grades_with_database() {
    let pool = setup_test_db().unwrap();

    let initial_count = count_salary_grades(&pool).unwrap();

    // Create test salary grades
    let sg1_id = create_test_salary_grade(&pool, "SG1", 30000.0).unwrap();
    let sg2_id = create_test_salary_grade(&pool, "SG2", 50000.0).unwrap();

    let final_count = count_salary_grades(&pool).unwrap();
    assert_eq!(
        final_count,
        initial_count + 2,
        "Salary grade count should increase by 2"
    );

    // Cleanup
    delete_test_salary_grade(&pool, &sg1_id).ok();
    delete_test_salary_grade(&pool, &sg2_id).ok();
}

#[test]
fn test_create_salary_grade_in_database() {
    let pool = setup_test_db().unwrap();

    let sg_id = create_test_salary_grade(&pool, "SG5", 80000.0).unwrap();

    assert!(
        salary_grade_exists(&pool, &sg_id).unwrap(),
        "Salary grade should exist in database"
    );

    // Cleanup
    delete_test_salary_grade(&pool, &sg_id).ok();
}

#[test]
fn test_salary_grade_with_different_codes() {
    let pool = setup_test_db().unwrap();

    let codes = vec!["A1", "B2", "C3", "D4", "E5"];
    let mut sg_ids = Vec::new();

    for code in &codes {
        let sg_id = create_test_salary_grade(&pool, code, 40000.0).unwrap();
        sg_ids.push(sg_id);
    }

    // Verify all exist
    for sg_id in &sg_ids {
        assert!(
            salary_grade_exists(&pool, sg_id).unwrap(),
            "Salary grade should exist"
        );
    }

    // Cleanup
    for sg_id in sg_ids {
        delete_test_salary_grade(&pool, &sg_id).ok();
    }
}

#[test]
fn test_salary_grade_with_various_base_salaries() {
    let pool = setup_test_db().unwrap();

    let salaries = [25000.0, 50000.0, 75000.0, 100000.0, 150000.0];
    let mut sg_ids = Vec::new();

    for (i, salary) in salaries.iter().enumerate() {
        let sg_id = create_test_salary_grade(&pool, &format!("SAL{}", i), *salary).unwrap();
        sg_ids.push(sg_id);
    }

    // Verify all exist
    for sg_id in &sg_ids {
        assert!(salary_grade_exists(&pool, sg_id).unwrap());
    }

    // Cleanup
    for sg_id in sg_ids {
        delete_test_salary_grade(&pool, &sg_id).ok();
    }
}

#[test]
fn test_multiple_salary_grades() {
    let pool = setup_test_db().unwrap();

    let initial_count = count_salary_grades(&pool).unwrap();

    // Create multiple salary grades
    let sg_ids: Vec<String> = (0..5)
        .map(|i| {
            create_test_salary_grade(
                &pool,
                &format!("GRADE{}", i),
                30000.0 + (i as f64 * 10000.0),
            )
            .unwrap()
        })
        .collect();

    let final_count = count_salary_grades(&pool).unwrap();
    assert_eq!(
        final_count,
        initial_count + 5,
        "Should have 5 more salary grades"
    );

    // Verify all exist
    for sg_id in &sg_ids {
        assert!(
            salary_grade_exists(&pool, sg_id).unwrap(),
            "Salary grade {} should exist",
            sg_id
        );
    }

    // Cleanup
    for sg_id in sg_ids {
        delete_test_salary_grade(&pool, &sg_id).ok();
    }
}

#[test]
fn test_create_salary_grade_request_validation() {
    // Verify the CreateSalaryGradeRequest structure
    let request = CreateSalaryGradeRequest {
        code: "SG10".to_string(),
        base_salary: 120000.0,
        description: Some("Executive Level".to_string()),
    };

    assert_eq!(request.code, "SG10");
    assert_eq!(request.description, Some("Executive Level".to_string()));
    assert_eq!(request.base_salary, 120000.0);
}

#[test]
fn test_update_salary_grade_request_structure() {
    // Verify the UpdateSalaryGradeRequest structure
    let request = UpdateSalaryGradeRequest {
        code: Some("SG11".to_string()),
        base_salary: Some(150000.0),
        description: Some("Director Level".to_string()),
    };

    assert!(request.code.is_some());
    assert!(request.description.is_some());
    assert!(request.base_salary.is_some());
}

#[test]
fn test_salary_grade_crud_operations() {
    let pool = setup_test_db().unwrap();

    let initial_count = count_salary_grades(&pool).unwrap();

    // CREATE
    let sg_id = create_test_salary_grade(&pool, "MANAGER", 90000.0).unwrap();
    assert!(
        salary_grade_exists(&pool, &sg_id).unwrap(),
        "Salary grade should be created"
    );

    // READ (verify count increased)
    let after_create_count = count_salary_grades(&pool).unwrap();
    assert_eq!(
        after_create_count,
        initial_count + 1,
        "Count should increase by 1"
    );

    // DELETE
    delete_test_salary_grade(&pool, &sg_id).ok();
    assert!(
        !salary_grade_exists(&pool, &sg_id).unwrap(),
        "Salary grade should be deleted"
    );

    // Verify count returned to initial
    let after_delete_count = count_salary_grades(&pool).unwrap();
    assert_eq!(
        after_delete_count, initial_count,
        "Count should return to initial"
    );
}

#[test]
fn test_salary_grade_count_tracking() {
    let pool = setup_test_db().unwrap();

    let initial_count = count_salary_grades(&pool).unwrap();

    // Add salary grades
    let sg1_id = create_test_salary_grade(&pool, "JUNIOR", 35000.0).unwrap();
    assert_eq!(count_salary_grades(&pool).unwrap(), initial_count + 1);

    let sg2_id = create_test_salary_grade(&pool, "SENIOR", 85000.0).unwrap();
    assert_eq!(count_salary_grades(&pool).unwrap(), initial_count + 2);

    // Remove salary grades
    delete_test_salary_grade(&pool, &sg1_id).ok();
    assert_eq!(count_salary_grades(&pool).unwrap(), initial_count + 1);

    delete_test_salary_grade(&pool, &sg2_id).ok();
    assert_eq!(count_salary_grades(&pool).unwrap(), initial_count);
}

#[test]
fn test_salary_grade_with_employees() {
    let pool = setup_test_db().unwrap();

    // Create a salary grade
    let sg_id = create_test_salary_grade(&pool, "LEAD", 70000.0).unwrap();

    // Create a department for employees
    let dept_id = create_test_department(&pool, "Engineering Dept").unwrap();

    // Create multiple employees with this salary grade
    let emp1_id = create_test_employee(
        &pool,
        "Charlie",
        "Brown",
        &format!("charlie_{}@test.com", uuid::Uuid::new_v4()),
        Some(&dept_id),
        Some(&sg_id),
    )
    .unwrap();

    let emp2_id = create_test_employee(
        &pool,
        "Diana",
        "Prince",
        &format!("diana_{}@test.com", uuid::Uuid::new_v4()),
        Some(&dept_id),
        Some(&sg_id),
    )
    .unwrap();

    // Verify salary grade and employees exist
    assert!(salary_grade_exists(&pool, &sg_id).unwrap());
    assert!(employee_exists(&pool, &emp1_id).unwrap());
    assert!(employee_exists(&pool, &emp2_id).unwrap());

    // Cleanup
    delete_test_employee(&pool, &emp1_id).ok();
    delete_test_employee(&pool, &emp2_id).ok();
    delete_test_department(&pool, &dept_id).ok();
    delete_test_salary_grade(&pool, &sg_id).ok();
}
