use crate::db::DbPool;
use crate::models::{CreateUserRequest, HealthResponse, User};
use actix_web::{HttpResponse, Responder, get, post, web};
use mysql::prelude::*;

// Health check endpoint
#[get("/health")]
pub async fn health() -> impl Responder {
    let response = HealthResponse {
        status: "ok".to_string(),
        message: "Server is running".to_string(),
    };
    HttpResponse::Ok().json(response)
}

// Database endpoint - Get all users
#[get("/api/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database connection error: {}", e)
            }));
        }
    };

    let result: Result<Vec<User>, mysql::Error> = conn
        .query_map("SELECT id, name, email FROM users", |(id, name, email)| {
            User { id, name, email }
        });

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

// Database endpoint - Get user by ID
#[get("/api/users/{id}")]
pub async fn get_user_by_id(pool: web::Data<DbPool>, id: web::Path<i32>) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database connection error: {}", e)
            }));
        }
    };

    let result: Result<Option<User>, mysql::Error> = conn
        .exec_first("SELECT id, name, email FROM users WHERE id = ?", (*id,))
        .map(|row: Option<(i32, String, String)>| {
            row.map(|(id, name, email)| User { id, name, email })
        });

    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

// Database endpoint - Create user
#[post("/api/users")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    user: web::Json<CreateUserRequest>,
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database connection error: {}", e)
            }));
        }
    };

    let result = conn.exec_drop(
        "INSERT INTO users (name, email) VALUES (?, ?)",
        (&user.name, &user.email),
    );

    match result {
        Ok(_) => {
            let user_id = conn.last_insert_id() as i32;
            HttpResponse::Created().json(serde_json::json!({
                "id": user_id,
                "name": user.name,
                "email": user.email
            }))
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}
