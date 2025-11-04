// Department management handlers

use crate::db::DbPool;
use crate::models::*;
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use mysql::prelude::*;
use uuid::Uuid;

// Type alias for department database row
type DepartmentRow = (
    String,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
);

/// Get all departments
#[utoipa::path(
    get,
    path = "/api/departments",
    responses(
        (status = 200, description = "List of all departments", body = Vec<Department>),
        (status = 500, description = "Internal server error")
    ),
    tag = "Departments"
)]
#[get("/api/departments")]
pub async fn get_departments(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database connection error: {}", e)
            }));
        }
    };

    let result: Result<Vec<Department>, mysql::Error> = conn.query_map(
        "SELECT id, name, head_id, created_at, updated_at FROM departments",
        |(id, name, head_id, created_at, updated_at)| Department {
            id,
            name,
            head_id,
            created_at,
            updated_at,
        },
    );

    match result {
        Ok(departments) => HttpResponse::Ok().json(departments),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

/// Get department by ID
#[utoipa::path(
    get,
    path = "/api/departments/{id}",
    params(
        ("id" = String, Path, description = "Department UUID")
    ),
    responses(
        (status = 200, description = "Department found", body = Department),
        (status = 404, description = "Department not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Departments"
)]
#[get("/api/departments/{id}")]
pub async fn get_department_by_id(
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

    let result: Result<Option<Department>, mysql::Error> = conn
        .exec_first(
            "SELECT id, name, head_id, created_at, updated_at FROM departments WHERE id = ?",
            (id.as_str(),),
        )
        .map(|row: Option<DepartmentRow>| {
            row.map(|(id, name, head_id, created_at, updated_at)| Department {
                id,
                name,
                head_id,
                created_at,
                updated_at,
            })
        });

    match result {
        Ok(Some(department)) => HttpResponse::Ok().json(department),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Department not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

/// Create new department
#[utoipa::path(
    post,
    path = "/api/departments",
    request_body = CreateDepartmentRequest,
    responses(
        (status = 201, description = "Department created successfully"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Departments"
)]
#[post("/api/departments")]
pub async fn create_department(
    pool: web::Data<DbPool>,
    department: web::Json<CreateDepartmentRequest>,
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
        "INSERT INTO departments (id, name, head_id) VALUES (?, ?, ?)",
        (&id, &department.name, &department.head_id),
    );

    match result {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({
            "id": id,
            "name": department.name,
            "head_id": department.head_id
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

/// Update department
#[utoipa::path(
    put,
    path = "/api/departments/{id}",
    params(
        ("id" = String, Path, description = "Department UUID")
    ),
    request_body = UpdateDepartmentRequest,
    responses(
        (status = 200, description = "Department updated successfully"),
        (status = 404, description = "Department not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Departments"
)]
#[put("/api/departments/{id}")]
pub async fn update_department(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
    department: web::Json<UpdateDepartmentRequest>,
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

    if let Some(ref name) = department.name {
        updates.push("name = ?");
        params.push(name.clone().into());
    }
    if let Some(ref head_id) = department.head_id {
        updates.push("head_id = ?");
        params.push(head_id.clone().into());
    }

    if updates.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No fields to update"
        }));
    }

    params.push(id.as_str().into());
    let query = format!("UPDATE departments SET {} WHERE id = ?", updates.join(", "));

    let result = conn.exec_drop(&query, params);

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Department updated successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

/// Delete department
#[utoipa::path(
    delete,
    path = "/api/departments/{id}",
    params(
        ("id" = String, Path, description = "Department UUID")
    ),
    responses(
        (status = 200, description = "Department deleted successfully"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Departments"
)]
#[delete("/api/departments/{id}")]
pub async fn delete_department(pool: web::Data<DbPool>, id: web::Path<String>) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database connection error: {}", e)
            }));
        }
    };

    let result = conn.exec_drop("DELETE FROM departments WHERE id = ?", (id.as_str(),));

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Department deleted successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}
