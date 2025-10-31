mod db;
mod handler;
mod handler_personnel;
mod models;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use handler::{create_user, get_user_by_id, get_users, health};
use handler_personnel::{
    // Employee endpoints
    assign_manager,
    assign_salary_grade,
    // Department endpoints
    create_department,
    create_employee,
    // Salary grade endpoints
    create_salary_grade,
    delete_department,
    delete_employee,
    delete_salary_grade,
    get_department_by_id,
    get_departments,
    get_employee_by_id,
    get_employees,
    get_employees_by_department,
    get_salary_grade_by_id,
    get_salary_grades,
    update_department,
    update_employee,
    update_salary_grade,
};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    // Create database connection pool
    let pool = db::create_pool(&database_url).expect("Failed to create database pool");

    println!("Database connected successfully!");
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // Health and legacy endpoints
            .service(health)
            .service(get_users)
            .service(get_user_by_id)
            .service(create_user)
            // Employee endpoints
            .service(get_employees)
            .service(get_employee_by_id)
            .service(create_employee)
            .service(update_employee)
            .service(delete_employee)
            .service(assign_manager)
            .service(assign_salary_grade)
            .service(get_employees_by_department)
            // Department endpoints
            .service(get_departments)
            .service(get_department_by_id)
            .service(create_department)
            .service(update_department)
            .service(delete_department)
            // Salary grade endpoints
            .service(get_salary_grades)
            .service(get_salary_grade_by_id)
            .service(create_salary_grade)
            .service(update_salary_grade)
            .service(delete_salary_grade)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
