// Salary grade management handlers

use crate::db::DbPool;
use crate::models::*;
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use mysql::prelude::*;
use uuid::Uuid;

// Type alias for salary grade database row
type SalaryGradeRow = (String, String, f64, Option<String>, Option<String>);

/// Get all salary grades
#[utoipa::path(
    get,
    path = "/api/salary-grades",
    responses(
        (status = 200, description = "List of all salary grades", body = Vec<SalaryGrade>),
        (status = 500, description = "Internal server error")
    ),
    tag = "Salary Grades"
)]
#[get("/api/salary-grades")]
pub async fn get_salary_grades(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database connection error: {}", e)
            }));
        }
    };

    let result: Result<Vec<SalaryGrade>, mysql::Error> = conn.query_map(
        "SELECT id, code, base_salary, description, created_at FROM salary_grades",
        |(id, code, base_salary, description, created_at)| SalaryGrade {
            id,
            code,
            base_salary,
            description,
            created_at,
        },
    );

    match result {
        Ok(grades) => HttpResponse::Ok().json(grades),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

/// Get salary grade by ID
#[utoipa::path(
    get,
    path = "/api/salary-grades/{id}",
    params(
        ("id" = String, Path, description = "Salary Grade UUID")
    ),
    responses(
        (status = 200, description = "Salary grade found", body = SalaryGrade),
        (status = 404, description = "Salary grade not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Salary Grades"
)]
#[get("/api/salary-grades/{id}")]
pub async fn get_salary_grade_by_id(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database connection error: {}", e)
            }));
        }
    };

    let result: Result<Option<SalaryGrade>, mysql::Error> = conn
        .exec_first(
            "SELECT id, code, base_salary, description, created_at FROM salary_grades WHERE id = ?",
            (id.as_str(),),
        )
        .map(|row: Option<SalaryGradeRow>| {
            row.map(
                |(id, code, base_salary, description, created_at)| SalaryGrade {
                    id,
                    code,
                    base_salary,
                    description,
                    created_at,
                },
            )
        });

    match result {
        Ok(Some(grade)) => HttpResponse::Ok().json(grade),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Salary grade not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

/// Create new salary grade
#[utoipa::path(
    post,
    path = "/api/salary-grades",
    request_body = CreateSalaryGradeRequest,
    responses(
        (status = 201, description = "Salary grade created successfully"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Salary Grades"
)]
#[post("/api/salary-grades")]
pub async fn create_salary_grade(
    pool: web::Data<DbPool>,
    grade: web::Json<CreateSalaryGradeRequest>,
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database connection error: {}", e)
            }));
        }
    };

    let id = Uuid::new_v4().to_string();

    let result = conn.exec_drop(
        "INSERT INTO salary_grades (id, code, base_salary, description) VALUES (?, ?, ?, ?)",
        (&id, &grade.code, grade.base_salary, &grade.description),
    );

    match result {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({
            "id": id,
            "code": grade.code,
            "base_salary": grade.base_salary,
            "description": grade.description
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

/// Update salary grade
#[utoipa::path(
    put,
    path = "/api/salary-grades/{id}",
    params(
        ("id" = String, Path, description = "Salary Grade UUID")
    ),
    request_body = UpdateSalaryGradeRequest,
    responses(
        (status = 200, description = "Salary grade updated successfully"),
        (status = 404, description = "Salary grade not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Salary Grades"
)]
#[put("/api/salary-grades/{id}")]
pub async fn update_salary_grade(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
    grade: web::Json<UpdateSalaryGradeRequest>,
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database connection error: {}", e)
            }));
        }
    };

    let mut updates = Vec::new();
    let mut params: Vec<mysql::Value> = Vec::new();

    if let Some(ref code) = grade.code {
        updates.push("code = ?");
        params.push(code.clone().into());
    }
    if let Some(base_salary) = grade.base_salary {
        updates.push("base_salary = ?");
        params.push(base_salary.into());
    }
    if let Some(ref description) = grade.description {
        updates.push("description = ?");
        params.push(description.clone().into());
    }

    if updates.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No fields to update"
        }));
    }

    params.push(id.as_str().into());
    let query = format!(
        "UPDATE salary_grades SET {} WHERE id = ?",
        updates.join(", ")
    );

    let result = conn.exec_drop(&query, params);

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Salary grade updated successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

/// Delete salary grade
#[utoipa::path(
    delete,
    path = "/api/salary-grades/{id}",
    params(
        ("id" = String, Path, description = "Salary Grade UUID")
    ),
    responses(
        (status = 200, description = "Salary grade deleted successfully"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Salary Grades"
)]
#[delete("/api/salary-grades/{id}")]
pub async fn delete_salary_grade(pool: web::Data<DbPool>, id: web::Path<String>) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database connection error: {}", e)
            }));
        }
    };

    let result = conn.exec_drop("DELETE FROM salary_grades WHERE id = ?", (id.as_str(),));

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Salary grade deleted successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}
