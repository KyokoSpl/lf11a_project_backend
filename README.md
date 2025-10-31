# Backend API - Personnel Management System (Personalverwaltung)

A comprehensive Rust backend API using Actix-web and MySQL for personnel management.

## Features

✅ **Employee Management** (Mitarbeiterverwaltung)
- Create, read, update, and delete employees
- Soft delete support (inactive employees)
- Assign managers to employees
- Assign salary grades to employees
- Filter by department

✅ **Department Management** (Abteilungsverwaltung)
- Create and manage departments
- Assign department heads
- View employees by department

✅ **Salary Grade Management** (Gehaltsstufen)
- Create and manage salary grades
- Assign grades to employees
- Track base salary for each grade

✅ **Role Management**
- Admin
- DepartmentHead (Abteilungsleiter)
- DeputyHead (Stellvertretender Leiter)
- Employee (Mitarbeiter)

✅ **Manager Assignment** (Vorgesetztenzuweisung)
- Assign managers to employees
- Hierarchical structure support

## Setup

### 1. Start MySQL with Docker

```bash
docker-compose up -d
```

This will:
- Start a MySQL 8.0 container
- Create a database called `mydb`
- Run the `init.sql` script to create tables with sample data
- Expose MySQL on port 3307

### 2. Install Dependencies

```bash
cargo build
```

### 3. Run the Server

```bash
cargo run
```

The server will start at `http://127.0.0.1:8080`

### 4. Run Tests

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test models_test
cargo test --test handler_test
cargo test --test handler_personnel_test
cargo test --test db_test

# Run with output
cargo test -- --nocapture
```

## API Endpoints

### Health Check
```bash
GET /health
```

### Employee Endpoints

#### Get All Employees
```bash
# Active employees only
GET /api/employees

# Include inactive employees
GET /api/employees?include_inactive=true
```

#### Get Employee by ID
```bash
GET /api/employees/{id}
```

#### Create Employee
```bash
POST /api/employees
Content-Type: application/json

{
  "first_name": "Max",
  "last_name": "Mustermann",
  "email": "max.mustermann@company.com",
  "department_id": "dept-uuid",
  "salary_grade_id": "grade-uuid",
  "manager_id": "manager-uuid",
  "role": "Employee",
  "hire_date": "2024-01-15"
}
```

Available roles: `Admin`, `DepartmentHead`, `DeputyHead`, `Employee`

#### Update Employee
```bash
PUT /api/employees/{id}
Content-Type: application/json

{
  "first_name": "Updated Name",
  "email": "new.email@company.com",
  "active": true
}
```

#### Delete Employee (Soft Delete)
```bash
DELETE /api/employees/{id}
```

#### Assign Manager
```bash
PUT /api/employees/{id}/manager
Content-Type: application/json

{
  "manager_id": "manager-uuid"
}
```

#### Assign Salary Grade
```bash
PUT /api/employees/{id}/salary-grade
Content-Type: application/json

{
  "salary_grade_id": "grade-uuid"
}
```

#### Get Employees by Department
```bash
GET /api/departments/{id}/employees
```

### Department Endpoints

#### Get All Departments
```bash
GET /api/departments
```

#### Get Department by ID
```bash
GET /api/departments/{id}
```

#### Create Department
```bash
POST /api/departments
Content-Type: application/json

{
  "name": "Engineering",
  "head_id": "employee-uuid"
}
```

#### Update Department
```bash
PUT /api/departments/{id}
Content-Type: application/json

{
  "name": "Updated Department Name",
  "head_id": "new-head-uuid"
}
```

#### Delete Department
```bash
DELETE /api/departments/{id}
```

### Salary Grade Endpoints

#### Get All Salary Grades
```bash
GET /api/salary-grades
```

#### Get Salary Grade by ID
```bash
GET /api/salary-grades/{id}
```

#### Create Salary Grade
```bash
POST /api/salary-grades
Content-Type: application/json

{
  "code": "E5",
  "base_salary": 100000.00,
  "description": "Expert Level"
}
```

#### Update Salary Grade
```bash
PUT /api/salary-grades/{id}
Content-Type: application/json

{
  "code": "E5-Updated",
  "base_salary": 105000.00,
  "description": "Expert Level - Updated"
}
```

#### Delete Salary Grade
```bash
DELETE /api/salary-grades/{id}
```

## Database Schema

### Tables

**departments**
- `id` (CHAR(36), Primary Key, UUID)
- `name` (VARCHAR(255), UNIQUE)
- `head_id` (CHAR(36), Foreign Key to employees)
- `created_at`, `updated_at`

**salary_grades**
- `id` (CHAR(36), Primary Key, UUID)
- `code` (VARCHAR(50), UNIQUE)
- `base_salary` (DECIMAL(12,2))
- `description` (TEXT)
- `created_at`

**employees**
- `id` (CHAR(36), Primary Key, UUID)
- `first_name`, `last_name` (VARCHAR(100))
- `email` (VARCHAR(255), UNIQUE)
- `department_id` (Foreign Key to departments)
- `salary_grade_id` (Foreign Key to salary_grades)
- `manager_id` (Self-referencing Foreign Key)
- `role` (ENUM: Admin, DepartmentHead, DeputyHead, Employee)
- `hire_date` (DATE)
- `active` (BOOLEAN)
- `deleted_at` (TIMESTAMP, for soft delete)
- `created_at`, `updated_at`

## Sample Data

The database is initialized with:
- 7 salary grades (E1-E4, M1-M2, D1)
- 6 departments (Engineering, HR, Marketing, Sales, Finance, IT Operations)
- 24 employees including:
  - 1 Admin
  - 6 Department Heads
  - 2 Deputy Heads
  - 15 Regular Employees

## Testing

The project includes comprehensive tests for:
- **Models** (`tests/models_test.rs`) - Data structure and serialization tests
- **Handlers** (`tests/handler_test.rs`) - Basic endpoint tests
- **Personnel Handlers** (`tests/handler_personnel_test.rs`) - Personnel management endpoint tests
- **Database** (`tests/db_test.rs`) - Database connection tests

Run tests with:
```bash
cargo test
```

## Docker Commands

```bash
# Start MySQL
docker-compose up -d

# Stop MySQL
docker-compose down

# View logs
docker-compose logs -f

# Restart with fresh data
docker-compose down -v
docker-compose up -d

# Access MySQL CLI
docker exec -it backend_mysql mysql -uroot -prootpassword mydb
```

## Project Structure

```
backend/
├── src/
│   ├── main.rs              # Server setup and routes
│   ├── lib.rs               # Library exports for tests
│   ├── handler.rs           # Basic/legacy handlers
│   ├── handler_personnel.rs # Personnel management handlers
│   ├── models.rs            # Data structures
│   └── db.rs                # Database connection
├── tests/
│   ├── models_test.rs       # Model tests
│   ├── handler_test.rs      # Handler tests
│   ├── handler_personnel_test.rs  # Personnel handler tests
│   └── db_test.rs           # Database tests
├── Cargo.toml               # Dependencies
├── docker-compose.yml       # MySQL Docker setup
├── init.sql                 # Database schema and seed data
├── .env                     # Environment variables
└── README.md                # This file
```

## Environment Variables

Create a `.env` file:
```
DATABASE_URL=mysql://root:rootpassword@localhost:3307/mydb
```

## Example Usage Scenarios

### Scenario 1: Create a new employee and assign them to a department with a manager

```bash
# 1. Create employee
curl -X POST http://127.0.0.1:8080/api/employees \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Anna",
    "last_name": "Schmidt",
    "email": "anna.schmidt@company.com",
    "department_id": "650e8400-e29b-41d4-a716-446655440001",
    "salary_grade_id": "550e8400-e29b-41d4-a716-446655440002",
    "role": "Employee",
    "hire_date": "2024-10-31"
  }'

# 2. Assign manager (use returned employee ID)
curl -X PUT http://127.0.0.1:8080/api/employees/{employee-id}/manager \
  -H "Content-Type: application/json" \
  -d '{"manager_id": "750e8400-e29b-41d4-a716-446655440002"}'
```

### Scenario 2: Promote an employee and change their salary grade

```bash
# Update role and salary grade
curl -X PUT http://127.0.0.1:8080/api/employees/{employee-id} \
  -H "Content-Type: application/json" \
  -d '{
    "role": "DeputyHead",
    "salary_grade_id": "550e8400-e29b-41d4-a716-446655440005"
  }'
```

### Scenario 3: Create a new department with a department head

```bash
# 1. Create department
curl -X POST http://127.0.0.1:8080/api/departments \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Product Management",
    "head_id": "750e8400-e29b-41d4-a716-446655440010"
  }'
```

## License

MIT
