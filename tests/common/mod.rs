// Test utilities for database integration tests

#![allow(dead_code)] // Test utilities may not all be used in every test file

use backend::db::create_pool;
use mysql::prelude::*;
use mysql::*;
use std::env;

pub fn get_test_db_url() -> String {
    env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://user:userpassword@127.0.0.1:3307/mydb".to_string())
}

pub fn setup_test_db() -> Result<Pool, mysql::Error> {
    let db_url = get_test_db_url();
    create_pool(&db_url)
}

pub fn cleanup_test_data(pool: &Pool) -> Result<(), mysql::Error> {
    let mut conn = pool.get_conn()?;

    // Clean up test data in reverse order of foreign keys
    conn.exec_drop("DELETE FROM employees WHERE email LIKE '%test%'", ())?;
    conn.exec_drop("DELETE FROM departments WHERE name LIKE '%Test%'", ())?;
    conn.exec_drop("DELETE FROM salary_grades WHERE code LIKE 'TEST%'", ())?;

    Ok(())
}

pub fn create_test_department(pool: &Pool, name: &str) -> Result<String, mysql::Error> {
    let mut conn = pool.get_conn()?;

    let id = uuid::Uuid::new_v4().to_string();
    // Add a UUID suffix to make the name unique
    let unique_name = format!(
        "{}_{}",
        name,
        uuid::Uuid::new_v4().to_string().split('-').next().unwrap()
    );
    conn.exec_drop(
        "INSERT INTO departments (id, name, head_id) VALUES (?, ?, NULL)",
        (&id, &unique_name),
    )?;

    Ok(id)
}

pub fn create_test_salary_grade(
    pool: &Pool,
    code: &str,
    base_salary: f64,
) -> Result<String, mysql::Error> {
    let mut conn = pool.get_conn()?;

    let id = uuid::Uuid::new_v4().to_string();
    // Add a UUID suffix to make the code unique
    let unique_code = format!(
        "{}_{}",
        code,
        uuid::Uuid::new_v4().to_string().split('-').next().unwrap()
    );
    conn.exec_drop(
        "INSERT INTO salary_grades (id, code, base_salary, description) VALUES (?, ?, ?, ?)",
        (&id, &unique_code, base_salary, "Test grade"),
    )?;

    Ok(id)
}

pub fn create_test_employee(
    pool: &Pool,
    first_name: &str,
    last_name: &str,
    email: &str,
    department_id: Option<&str>,
    salary_grade_id: Option<&str>,
) -> Result<String, mysql::Error> {
    let mut conn = pool.get_conn()?;

    let id = uuid::Uuid::new_v4().to_string();
    conn.exec_drop(
        "INSERT INTO employees (id, first_name, last_name, email, department_id, salary_grade_id, role) VALUES (?, ?, ?, ?, ?, ?, 'Employee')",
        (&id, first_name, last_name, email, department_id, salary_grade_id),
    )?;

    Ok(id)
}

pub fn count_employees(pool: &Pool) -> Result<usize, mysql::Error> {
    let mut conn = pool.get_conn()?;
    let count: Option<usize> = conn.query_first("SELECT COUNT(*) FROM employees")?;
    Ok(count.unwrap_or(0))
}

pub fn count_departments(pool: &Pool) -> Result<usize, mysql::Error> {
    let mut conn = pool.get_conn()?;
    let count: Option<usize> = conn.query_first("SELECT COUNT(*) FROM departments")?;
    Ok(count.unwrap_or(0))
}

pub fn count_salary_grades(pool: &Pool) -> Result<usize, mysql::Error> {
    let mut conn = pool.get_conn()?;
    let count: Option<usize> = conn.query_first("SELECT COUNT(*) FROM salary_grades")?;
    Ok(count.unwrap_or(0))
}

pub fn get_employee_by_email(pool: &Pool, email: &str) -> Result<Option<String>, mysql::Error> {
    let mut conn = pool.get_conn()?;
    let id: Option<String> =
        conn.exec_first("SELECT id FROM employees WHERE email = ?", (email,))?;
    Ok(id)
}

pub fn get_department_by_name(pool: &Pool, name: &str) -> Result<Option<String>, mysql::Error> {
    let mut conn = pool.get_conn()?;
    let id: Option<String> =
        conn.exec_first("SELECT id FROM departments WHERE name = ?", (name,))?;
    Ok(id)
}

pub fn get_salary_grade_by_code(pool: &Pool, code: &str) -> Result<Option<String>, mysql::Error> {
    let mut conn = pool.get_conn()?;
    let id: Option<String> =
        conn.exec_first("SELECT id FROM salary_grades WHERE code = ?", (code,))?;
    Ok(id)
}

pub fn delete_test_employee(pool: &Pool, id: &str) -> Result<(), mysql::Error> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop("DELETE FROM employees WHERE id = ?", (id,))?;
    Ok(())
}

pub fn delete_test_department(pool: &Pool, id: &str) -> Result<(), mysql::Error> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop("DELETE FROM departments WHERE id = ?", (id,))?;
    Ok(())
}

pub fn delete_test_salary_grade(pool: &Pool, id: &str) -> Result<(), mysql::Error> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop("DELETE FROM salary_grades WHERE id = ?", (id,))?;
    Ok(())
}

pub fn employee_exists(pool: &Pool, id: &str) -> Result<bool, mysql::Error> {
    let mut conn = pool.get_conn()?;
    let count: Option<usize> =
        conn.exec_first("SELECT COUNT(*) FROM employees WHERE id = ?", (id,))?;
    Ok(count.unwrap_or(0) > 0)
}

pub fn department_exists(pool: &Pool, id: &str) -> Result<bool, mysql::Error> {
    let mut conn = pool.get_conn()?;
    let count: Option<usize> =
        conn.exec_first("SELECT COUNT(*) FROM departments WHERE id = ?", (id,))?;
    Ok(count.unwrap_or(0) > 0)
}

pub fn salary_grade_exists(pool: &Pool, id: &str) -> Result<bool, mysql::Error> {
    let mut conn = pool.get_conn()?;
    let count: Option<usize> =
        conn.exec_first("SELECT COUNT(*) FROM salary_grades WHERE id = ?", (id,))?;
    Ok(count.unwrap_or(0) > 0)
}
