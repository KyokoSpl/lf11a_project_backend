// Employee management handlers

use crate::db::DbPool;
use crate::models::*;
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use mysql::prelude::*;
use uuid::Uuid;

/// Get all employees (active only by default)
#[utoipa::path(
    get,
    path = "/api/employees",
    params(
        ("include_inactive" = Option<String>, Query, description = "Include inactive employees (true/false)")
    ),
    responses(
        (status = 200, description = "List of employees", body = Vec<Employee>),
        (status = 500, description = "Internal server error")
    ),
    tag = "Employees"
)]
#[get("/api/employees")]
pub async fn get_employees(
    pool: web::Data<DbPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database connection error: {}", e)
            }));
        }
    };

    let include_inactive = query
        .get("include_inactive")
        .map(|v| v == "true")
        .unwrap_or(false);

    let query_str = if include_inactive {
        "SELECT id, first_name, last_name, email, department_id, salary_grade_id, manager_id, role, hire_date, active, deleted_at, created_at, updated_at FROM employees"
    } else {
        "SELECT id, first_name, last_name, email, department_id, salary_grade_id, manager_id, role, hire_date, active, deleted_at, created_at, updated_at FROM employees WHERE active = TRUE"
    };

    let rows: Result<Vec<mysql::Row>, mysql::Error> = conn.query(query_str);

    match rows {
        Ok(rows) => {
            let employees: Vec<Employee> = rows
                .into_iter()
                .map(|mut row| Employee {
                    id: row.take("id").unwrap(),
                    first_name: row.take("first_name").unwrap(),
                    last_name: row.take("last_name").unwrap(),
                    email: row.take("email").unwrap(),
                    department_id: row.take("department_id").unwrap(),
                    salary_grade_id: row.take("salary_grade_id").unwrap(),
                    manager_id: row.take("manager_id").unwrap(),
                    role: row.take("role").unwrap(),
                    hire_date: row.take("hire_date").unwrap(),
                    active: row.take("active").unwrap(),
                    deleted_at: row.take("deleted_at").unwrap(),
                    created_at: row.take("created_at").unwrap(),
                    updated_at: row.take("updated_at").unwrap(),
                })
                .collect();
            HttpResponse::Ok().json(employees)
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

/// Get employee by ID
#[utoipa::path(
    get,
    path = "/api/employees/{id}",
    params(
        ("id" = String, Path, description = "Employee UUID")
    ),
    responses(
        (status = 200, description = "Employee found", body = Employee),
        (status = 404, description = "Employee not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Employees"
)]
#[get("/api/employees/{id}")]
pub async fn get_employee_by_id(pool: web::Data<DbPool>, id: web::Path<String>) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database connection error: {}", e)
            }));
        }
    };

    let rows: Result<Vec<mysql::Row>, mysql::Error> = conn.exec(
        "SELECT id, first_name, last_name, email, department_id, salary_grade_id, manager_id, role, hire_date, active, deleted_at, created_at, updated_at FROM employees WHERE id = ?",
        (id.as_str(),)
    );

    match rows {
        Ok(mut rows) => {
            if let Some(mut row) = rows.pop() {
                let employee = Employee {
                    id: row.take("id").unwrap(),
                    first_name: row.take("first_name").unwrap(),
                    last_name: row.take("last_name").unwrap(),
                    email: row.take("email").unwrap(),
                    department_id: row.take("department_id").unwrap(),
                    salary_grade_id: row.take("salary_grade_id").unwrap(),
                    manager_id: row.take("manager_id").unwrap(),
                    role: row.take("role").unwrap(),
                    hire_date: row.take("hire_date").unwrap(),
                    active: row.take("active").unwrap(),
                    deleted_at: row.take("deleted_at").unwrap(),
                    created_at: row.take("created_at").unwrap(),
                    updated_at: row.take("updated_at").unwrap(),
                };
                HttpResponse::Ok().json(employee)
            } else {
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "Employee not found"
                }))
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

/// Create new employee
#[utoipa::path(
    post,
    path = "/api/employees",
    request_body = CreateEmployeeRequest,
    responses(
        (status = 201, description = "Employee created successfully"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Employees"
)]
#[post("/api/employees")]
pub async fn create_employee(
    pool: web::Data<DbPool>,
    employee: web::Json<CreateEmployeeRequest>,
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
    let role = employee.role.as_deref().unwrap_or("Employee");

    let result = conn.exec_drop(
        "INSERT INTO employees (id, first_name, last_name, email, department_id, salary_grade_id, manager_id, role, hire_date) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        (&id, &employee.first_name, &employee.last_name, &employee.email, &employee.department_id, &employee.salary_grade_id, &employee.manager_id, role, &employee.hire_date)
    );

    match result {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({
            "id": id,
            "first_name": employee.first_name,
            "last_name": employee.last_name,
            "email": employee.email,
            "role": role
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

/// Update employee
#[utoipa::path(
    put,
    path = "/api/employees/{id}",
    params(
        ("id" = String, Path, description = "Employee UUID")
    ),
    request_body = UpdateEmployeeRequest,
    responses(
        (status = 200, description = "Employee updated successfully"),
        (status = 404, description = "Employee not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Employees"
)]
#[put("/api/employees/{id}")]
pub async fn update_employee(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
    employee: web::Json<UpdateEmployeeRequest>,
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

    if let Some(ref first_name) = employee.first_name {
        updates.push("first_name = ?");
        params.push(first_name.clone().into());
    }
    if let Some(ref last_name) = employee.last_name {
        updates.push("last_name = ?");
        params.push(last_name.clone().into());
    }
    if let Some(ref email) = employee.email {
        updates.push("email = ?");
        params.push(email.clone().into());
    }
    if let Some(ref department_id) = employee.department_id {
        updates.push("department_id = ?");
        params.push(department_id.clone().into());
    }
    if let Some(ref salary_grade_id) = employee.salary_grade_id {
        updates.push("salary_grade_id = ?");
        params.push(salary_grade_id.clone().into());
    }
    if let Some(ref manager_id) = employee.manager_id {
        updates.push("manager_id = ?");
        params.push(manager_id.clone().into());
    }
    if let Some(ref role) = employee.role {
        updates.push("role = ?");
        params.push(role.clone().into());
    }
    if let Some(ref hire_date) = employee.hire_date {
        updates.push("hire_date = ?");
        params.push(hire_date.clone().into());
    }
    if let Some(active) = employee.active {
        updates.push("active = ?");
        params.push(active.into());
    }

    if updates.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No fields to update"
        }));
    }

    params.push(id.as_str().into());
    let query = format!("UPDATE employees SET {} WHERE id = ?", updates.join(", "));

    let result = conn.exec_drop(&query, params);

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Employee updated successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

/// Delete employee (soft delete)
#[utoipa::path(
    delete,
    path = "/api/employees/{id}",
    params(
        ("id" = String, Path, description = "Employee UUID")
    ),
    responses(
        (status = 200, description = "Employee deleted successfully"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Employees"
)]
#[delete("/api/employees/{id}")]
pub async fn delete_employee(pool: web::Data<DbPool>, id: web::Path<String>) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Database connection error: {}", e)
            }));
        }
    };

    let result = conn.exec_drop(
        "UPDATE employees SET active = FALSE, deleted_at = NOW() WHERE id = ?",
        (id.as_str(),),
    );

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Employee deleted successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

/// Assign manager to employee
#[utoipa::path(
    put,
    path = "/api/employees/{id}/manager",
    params(
        ("id" = String, Path, description = "Employee UUID")
    ),
    request_body = AssignManagerRequest,
    responses(
        (status = 200, description = "Manager assigned successfully"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Employees"
)]
#[put("/api/employees/{id}/manager")]
pub async fn assign_manager(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
    req: web::Json<AssignManagerRequest>,
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
        "UPDATE employees SET manager_id = ? WHERE id = ?",
        (&req.manager_id, id.as_str()),
    );

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Manager assigned successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

/// Assign salary grade to employee
#[utoipa::path(
    put,
    path = "/api/employees/{id}/salary-grade",
    params(
        ("id" = String, Path, description = "Employee UUID")
    ),
    request_body = AssignSalaryGradeRequest,
    responses(
        (status = 200, description = "Salary grade assigned successfully"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Employees"
)]
#[put("/api/employees/{id}/salary-grade")]
pub async fn assign_salary_grade(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
    req: web::Json<AssignSalaryGradeRequest>,
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
        "UPDATE employees SET salary_grade_id = ? WHERE id = ?",
        (&req.salary_grade_id, id.as_str()),
    );

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Salary grade assigned successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

/// Get employees by department
#[utoipa::path(
    get,
    path = "/api/departments/{id}/employees",
    params(
        ("id" = String, Path, description = "Department UUID")
    ),
    responses(
        (status = 200, description = "List of employees in department", body = Vec<Employee>),
        (status = 500, description = "Internal server error")
    ),
    tag = "Employees"
)]
#[get("/api/departments/{id}/employees")]
pub async fn get_employees_by_department(
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

    let rows: Result<Vec<mysql::Row>, mysql::Error> = conn.exec(
        "SELECT id, first_name, last_name, email, department_id, salary_grade_id, manager_id, role, hire_date, active, deleted_at, created_at, updated_at FROM employees WHERE department_id = ? AND active = TRUE",
        (id.as_str(),)
    );

    match rows {
        Ok(rows) => {
            let employees: Vec<Employee> = rows
                .into_iter()
                .map(|mut row| Employee {
                    id: row.take("id").unwrap(),
                    first_name: row.take("first_name").unwrap(),
                    last_name: row.take("last_name").unwrap(),
                    email: row.take("email").unwrap(),
                    department_id: row.take("department_id").unwrap(),
                    salary_grade_id: row.take("salary_grade_id").unwrap(),
                    manager_id: row.take("manager_id").unwrap(),
                    role: row.take("role").unwrap(),
                    hire_date: row.take("hire_date").unwrap(),
                    active: row.take("active").unwrap(),
                    deleted_at: row.take("deleted_at").unwrap(),
                    created_at: row.take("created_at").unwrap(),
                    updated_at: row.take("updated_at").unwrap(),
                })
                .collect();
            HttpResponse::Ok().json(employees)
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}
