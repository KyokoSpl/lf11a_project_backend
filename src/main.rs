// Main File where the .env data is read and the actix api aka the http server is created

mod db;
mod handler;
mod handlers;
mod models;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use handler::{create_user, get_user_by_id, get_users, health};
use handlers::{
    // Employee endpoints
    assign_manager,
    assign_salary_grade,
    create_employee,
    delete_employee,
    get_employee_by_id,
    get_employees,
    get_employees_by_department,
    update_employee,
    // Department endpoints
    create_department,
    delete_department,
    get_department_by_id,
    get_departments,
    update_department,
    // Salary grade endpoints
    create_salary_grade,
    delete_salary_grade,
    get_salary_grade_by_id,
    get_salary_grades,
    update_salary_grade,
};
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Health and Users
        handler::health,
        handler::get_users,
        handler::get_user_by_id,
        handler::create_user,
        // Employees
        handlers::employee::get_employees,
        handlers::employee::get_employee_by_id,
        handlers::employee::create_employee,
        handlers::employee::update_employee,
        handlers::employee::delete_employee,
        handlers::employee::assign_manager,
        handlers::employee::assign_salary_grade,
        handlers::employee::get_employees_by_department,
        // Departments
        handlers::department::get_departments,
        handlers::department::get_department_by_id,
        handlers::department::create_department,
        handlers::department::update_department,
        handlers::department::delete_department,
        // Salary Grades
        handlers::salary_grade::get_salary_grades,
        handlers::salary_grade::get_salary_grade_by_id,
        handlers::salary_grade::create_salary_grade,
        handlers::salary_grade::update_salary_grade,
        handlers::salary_grade::delete_salary_grade,
    ),
    components(
        schemas(
            models::HealthResponse,
            models::User,
            models::CreateUserRequest,
            models::Employee,
            models::CreateEmployeeRequest,
            models::UpdateEmployeeRequest,
            models::AssignManagerRequest,
            models::AssignSalaryGradeRequest,
            models::Department,
            models::CreateDepartmentRequest,
            models::UpdateDepartmentRequest,
            models::SalaryGrade,
            models::CreateSalaryGradeRequest,
            models::UpdateSalaryGradeRequest,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Users", description = "User management endpoints"),
        (name = "Employees", description = "Employee management endpoints"),
        (name = "Departments", description = "Department management endpoints"),
        (name = "Salary Grades", description = "Salary grade management endpoints")
    ),
    info(
        title = "Personnel Management API",
        version = "1.0.0",
        description = "API for managing employees, departments, and salary grades",
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    // Create database connection pool
    let pool = db::create_pool(&database_url).expect("Failed to create database pool");

    println!("Database connected successfully!");
    println!("Starting server at http://{}:{}", host, port);
    println!("Swagger UI available at http://{}:{}/docs/", host, port);

    // Generate OpenAPI spec
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // Swagger UI
            .service(
                SwaggerUi::new("/docs/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone())
            )
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
    .bind((host.as_str(), port))?
    .run()
    .await
}
