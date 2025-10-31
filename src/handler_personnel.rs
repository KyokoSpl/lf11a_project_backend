use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use crate::models::*;
use crate::db::DbPool;
use mysql::prelude::*;
use uuid::Uuid;

// ==================== EMPLOYEE ENDPOINTS ====================

/// Get all employees (active only by default)
#[get("/api/employees")]
pub async fn get_employees(
    pool: web::Data<DbPool>,
    query: web::Query<std::collections::HashMap<String, String>>
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database connection error: {}", e)
        }))
    };

    let include_inactive = query.get("include_inactive").map(|v| v == "true").unwrap_or(false);
    
    let query_str = if include_inactive {
        "SELECT id, first_name, last_name, email, department_id, salary_grade_id, manager_id, role, hire_date, active, deleted_at, created_at, updated_at FROM employees"
    } else {
        "SELECT id, first_name, last_name, email, department_id, salary_grade_id, manager_id, role, hire_date, active, deleted_at, created_at, updated_at FROM employees WHERE active = TRUE"
    };

    let rows: Result<Vec<mysql::Row>, mysql::Error> = conn.query(query_str);
    
    match rows {
        Ok(rows) => {
            let employees: Vec<Employee> = rows.into_iter().map(|mut row| {
                Employee {
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
                }
            }).collect();
            HttpResponse::Ok().json(employees)
        },
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        }))
    }
}

/// Get employee by ID
#[get("/api/employees/{id}")]
pub async fn get_employee_by_id(
    pool: web::Data<DbPool>,
    id: web::Path<String>
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database connection error: {}", e)
        }))
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
        },
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        }))
    }
}

/// Create new employee
#[post("/api/employees")]
pub async fn create_employee(
    pool: web::Data<DbPool>,
    employee: web::Json<CreateEmployeeRequest>
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database connection error: {}", e)
        }))
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
        }))
    }
}

/// Update employee
#[put("/api/employees/{id}")]
pub async fn update_employee(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
    employee: web::Json<UpdateEmployeeRequest>
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database connection error: {}", e)
        }))
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
        }))
    }
}

/// Delete employee (soft delete)
#[delete("/api/employees/{id}")]
pub async fn delete_employee(
    pool: web::Data<DbPool>,
    id: web::Path<String>
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database connection error: {}", e)
        }))
    };

    let result = conn.exec_drop(
        "UPDATE employees SET active = FALSE, deleted_at = NOW() WHERE id = ?",
        (id.as_str(),)
    );

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Employee deleted successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        }))
    }
}

/// Assign manager to employee
#[put("/api/employees/{id}/manager")]
pub async fn assign_manager(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
    req: web::Json<AssignManagerRequest>
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database connection error: {}", e)
        }))
    };

    let result = conn.exec_drop(
        "UPDATE employees SET manager_id = ? WHERE id = ?",
        (&req.manager_id, id.as_str())
    );

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Manager assigned successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        }))
    }
}

/// Assign salary grade to employee
#[put("/api/employees/{id}/salary-grade")]
pub async fn assign_salary_grade(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
    req: web::Json<AssignSalaryGradeRequest>
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database connection error: {}", e)
        }))
    };

    let result = conn.exec_drop(
        "UPDATE employees SET salary_grade_id = ? WHERE id = ?",
        (&req.salary_grade_id, id.as_str())
    );

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Salary grade assigned successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        }))
    }
}

/// Get employees by department
#[get("/api/departments/{id}/employees")]
pub async fn get_employees_by_department(
    pool: web::Data<DbPool>,
    id: web::Path<String>
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database connection error: {}", e)
        }))
    };

    let rows: Result<Vec<mysql::Row>, mysql::Error> = conn.exec(
        "SELECT id, first_name, last_name, email, department_id, salary_grade_id, manager_id, role, hire_date, active, deleted_at, created_at, updated_at FROM employees WHERE department_id = ? AND active = TRUE",
        (id.as_str(),)
    );

    match rows {
        Ok(rows) => {
            let employees: Vec<Employee> = rows.into_iter().map(|mut row| {
                Employee {
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
                }
            }).collect();
            HttpResponse::Ok().json(employees)
        },
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        }))
    }
}

// ==================== DEPARTMENT ENDPOINTS ====================

/// Get all departments
#[get("/api/departments")]
pub async fn get_departments(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database connection error: {}", e)
        }))
    };

    let result: Result<Vec<Department>, mysql::Error> = conn.query_map(
        "SELECT id, name, head_id, created_at, updated_at FROM departments",
        |(id, name, head_id, created_at, updated_at)| Department { id, name, head_id, created_at, updated_at }
    );

    match result {
        Ok(departments) => HttpResponse::Ok().json(departments),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        }))
    }
}

/// Get department by ID
#[get("/api/departments/{id}")]
pub async fn get_department_by_id(
    pool: web::Data<DbPool>,
    id: web::Path<String>
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database connection error: {}", e)
        }))
    };

    let result: Result<Option<Department>, mysql::Error> = conn.exec_first(
        "SELECT id, name, head_id, created_at, updated_at FROM departments WHERE id = ?",
        (id.as_str(),)
    ).map(|row: Option<(String, String, Option<String>, Option<String>, Option<String>)>| {
        row.map(|(id, name, head_id, created_at, updated_at)| Department { id, name, head_id, created_at, updated_at })
    });

    match result {
        Ok(Some(department)) => HttpResponse::Ok().json(department),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Department not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        }))
    }
}

/// Create new department
#[post("/api/departments")]
pub async fn create_department(
    pool: web::Data<DbPool>,
    department: web::Json<CreateDepartmentRequest>
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database connection error: {}", e)
        }))
    };

    let id = Uuid::new_v4().to_string();

    let result = conn.exec_drop(
        "INSERT INTO departments (id, name, head_id) VALUES (?, ?, ?)",
        (&id, &department.name, &department.head_id)
    );

    match result {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({
            "id": id,
            "name": department.name,
            "head_id": department.head_id
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        }))
    }
}

/// Update department
#[put("/api/departments/{id}")]
pub async fn update_department(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
    department: web::Json<UpdateDepartmentRequest>
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database connection error: {}", e)
        }))
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
        }))
    }
}

/// Delete department
#[delete("/api/departments/{id}")]
pub async fn delete_department(
    pool: web::Data<DbPool>,
    id: web::Path<String>
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database connection error: {}", e)
        }))
    };

    let result = conn.exec_drop(
        "DELETE FROM departments WHERE id = ?",
        (id.as_str(),)
    );

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Department deleted successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        }))
    }
}

// ==================== SALARY GRADE ENDPOINTS ====================

/// Get all salary grades
#[get("/api/salary-grades")]
pub async fn get_salary_grades(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database connection error: {}", e)
        }))
    };

    let result: Result<Vec<SalaryGrade>, mysql::Error> = conn.query_map(
        "SELECT id, code, base_salary, description, created_at FROM salary_grades",
        |(id, code, base_salary, description, created_at)| SalaryGrade { id, code, base_salary, description, created_at }
    );

    match result {
        Ok(grades) => HttpResponse::Ok().json(grades),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        }))
    }
}

/// Get salary grade by ID
#[get("/api/salary-grades/{id}")]
pub async fn get_salary_grade_by_id(
    pool: web::Data<DbPool>,
    id: web::Path<String>
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database connection error: {}", e)
        }))
    };

    let result: Result<Option<SalaryGrade>, mysql::Error> = conn.exec_first(
        "SELECT id, code, base_salary, description, created_at FROM salary_grades WHERE id = ?",
        (id.as_str(),)
    ).map(|row: Option<(String, String, f64, Option<String>, Option<String>)>| {
        row.map(|(id, code, base_salary, description, created_at)| SalaryGrade { id, code, base_salary, description, created_at })
    });

    match result {
        Ok(Some(grade)) => HttpResponse::Ok().json(grade),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Salary grade not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        }))
    }
}

/// Create new salary grade
#[post("/api/salary-grades")]
pub async fn create_salary_grade(
    pool: web::Data<DbPool>,
    grade: web::Json<CreateSalaryGradeRequest>
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database connection error: {}", e)
        }))
    };

    let id = Uuid::new_v4().to_string();

    let result = conn.exec_drop(
        "INSERT INTO salary_grades (id, code, base_salary, description) VALUES (?, ?, ?, ?)",
        (&id, &grade.code, grade.base_salary, &grade.description)
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
        }))
    }
}

/// Update salary grade
#[put("/api/salary-grades/{id}")]
pub async fn update_salary_grade(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
    grade: web::Json<UpdateSalaryGradeRequest>
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database connection error: {}", e)
        }))
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
    let query = format!("UPDATE salary_grades SET {} WHERE id = ?", updates.join(", "));

    let result = conn.exec_drop(&query, params);

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Salary grade updated successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        }))
    }
}

/// Delete salary grade
#[delete("/api/salary-grades/{id}")]
pub async fn delete_salary_grade(
    pool: web::Data<DbPool>,
    id: web::Path<String>
) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database connection error: {}", e)
        }))
    };

    let result = conn.exec_drop(
        "DELETE FROM salary_grades WHERE id = ?",
        (id.as_str(),)
    );

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Salary grade deleted successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        }))
    }
}
