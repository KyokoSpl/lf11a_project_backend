// Models used for Api and database connection (everything that has option, can be set but doesn't
// have to and could stay null or not set if wanted)

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

// Department Models
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct Department {
    pub id: String,
    pub name: String,
    pub head_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateDepartmentRequest {
    pub name: String,
    pub head_id: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateDepartmentRequest {
    pub name: Option<String>,
    pub head_id: Option<String>,
}

// Salary Grade Models
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct SalaryGrade {
    pub id: String,
    pub code: String,
    pub base_salary: f64,
    pub description: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateSalaryGradeRequest {
    pub code: String,
    pub base_salary: f64,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateSalaryGradeRequest {
    pub code: Option<String>,
    pub base_salary: Option<f64>,
    pub description: Option<String>,
}

// Employee Models
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct Employee {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub department_id: Option<String>,
    pub salary_grade_id: Option<String>,
    pub manager_id: Option<String>,
    pub role: String,
    pub hire_date: Option<String>,
    pub active: bool,
    pub deleted_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateEmployeeRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub department_id: Option<String>,
    pub salary_grade_id: Option<String>,
    pub manager_id: Option<String>,
    pub role: Option<String>,
    pub hire_date: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateEmployeeRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub department_id: Option<String>,
    pub salary_grade_id: Option<String>,
    pub manager_id: Option<String>,
    pub role: Option<String>,
    pub hire_date: Option<String>,
    pub active: Option<bool>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AssignManagerRequest {
    pub manager_id: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AssignSalaryGradeRequest {
    pub salary_grade_id: String,
}
