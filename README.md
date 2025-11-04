# Backend API - Personnel Management System (Personalverwaltung)

[![CI](https://github.com/KyokoSpl/lf11a_project_backend/actions/workflows/ci.yml/badge.svg)](https://github.com/KyokoSpl/lf11a_project_backend/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/KyokoSpl/lf11a_project_backend/badge.svg?branch=main&kill_cache=1)](https://coveralls.io/github/KyokoSpl/lf11a_project_backend?branch=main)

A comprehensive Rust backend API using Actix-web and MySQL for personnel management with OpenAPI/Swagger documentation.

## Requirements

### Rust

#### Installation:

- Windows:
  - [.exe download](https://rust-lang.org/tools/install/?platform_override=win)
  - [More Docs](https://rust-lang.github.io/rustup/installation/windows.html)

- Linux/UNIX/MacOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

### Docker

#### Installation:

- Windows:
  [Docs](https://docs.docker.com/desktop/setup/install/windows-install/)
- Linux:
  - use your package manager and the this [Docs](https://docs.docker.com/desktop/setup/install/linux/)

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

✅ **OpenAPI/Swagger Documentation**

- Interactive API documentation at `/docs`
- Try out endpoints directly from the browser
- Complete API specifications with request/response examples

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

### 2. Configure Environment Variables

Create a `.env` file in the project root:

```env
# Database Configuration
DATABASE_URL=mysql://user:userpassword@localhost:3307/mydb

# Server Configuration
HOST=127.0.0.1
PORT=8080
```

### 3. Install Dependencies

```bash
cargo build
```

### 4. Run the Server

```bash
cargo run
```

The server will start at `http://127.0.0.1:8080`

**🎉 Access the interactive API documentation at: `http://127.0.0.1:8080/docs/`**

### 5. Run Tests

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

## API Documentation

### Interactive Swagger UI

The API includes comprehensive OpenAPI 3.0 documentation accessible through Swagger UI.

**Access at: `http://127.0.0.1:8080/docs/`**

#### Features:

- 📖 **Complete API Reference** - All endpoints documented with descriptions
- 🧪 **Try It Out** - Test endpoints directly from your browser
- 📝 **Request/Response Examples** - See example payloads for all operations
- 🏷️ **Organized by Tags** - Endpoints grouped by domain:
  - Health - System health check
  - Users - Legacy user management
  - Employees - Employee management (CRUD, assignments)
  - Departments - Department management
  - Salary Grades - Salary grade management
- 📊 **Schema Definitions** - Detailed data models for all requests/responses

#### Available Endpoints:

**Health Check**
- `GET /health` - Check server status

**Employees** (Mitarbeiter)
- `GET /api/employees` - List all employees (with optional inactive filter)
- `GET /api/employees/{id}` - Get employee details
- `POST /api/employees` - Create new employee
- `PUT /api/employees/{id}` - Update employee
- `DELETE /api/employees/{id}` - Delete employee (soft delete)
- `PUT /api/employees/{id}/manager` - Assign manager
- `PUT /api/employees/{id}/salary-grade` - Assign salary grade
- `GET /api/departments/{id}/employees` - Get employees by department

**Departments** (Abteilungen)
- `GET /api/departments` - List all departments
- `GET /api/departments/{id}` - Get department details
- `POST /api/departments` - Create new department
- `PUT /api/departments/{id}` - Update department
- `DELETE /api/departments/{id}` - Delete department

**Salary Grades** (Gehaltsstufen)
- `GET /api/salary-grades` - List all salary grades
- `GET /api/salary-grades/{id}` - Get salary grade details
- `POST /api/salary-grades` - Create new salary grade
- `PUT /api/salary-grades/{id}` - Update salary grade
- `DELETE /api/salary-grades/{id}` - Delete salary grade

**Legacy Users**
- `GET /api/users` - List all users
- `GET /api/users/{id}` - Get user by ID
- `POST /api/users` - Create new user

### OpenAPI Specification

The raw OpenAPI specification is available at:
- **JSON Format**: `http://127.0.0.1:8080/api-docs/openapi.json`

This can be imported into tools like Postman, Insomnia, or used to generate client SDKs.

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

The project includes comprehensive test coverage (31%) with multiple test types:

### Test Organization

**Unit Tests:**
- `tests/unit_models_test.rs` - Data structure and serialization tests (25 tests)
- `tests/handlers_module_test.rs` - Request/response model validation (19 tests)

**Integration Tests (HTTP Endpoints):**
- `tests/integration_employee_test.rs` - Employee endpoint structure tests (7 tests)
- `tests/integration_department_test.rs` - Department endpoint structure tests (4 tests)
- `tests/integration_salary_grade_test.rs` - Salary grade endpoint structure tests (4 tests)
- `tests/integration_handler_test.rs` - Legacy handler endpoint tests (5 tests)
- `tests/direct_handler_test.rs` - Basic handler tests (3 tests)

**Database Integration Tests:**
- `tests/db_employee_integration_test.rs` - Employee database operations (13 tests)
- `tests/db_department_integration_test.rs` - Department database operations (9 tests)
- `tests/db_salary_grade_integration_test.rs` - Salary grade database operations (10 tests)
- `tests/db_test.rs` - Database connection tests

**HTTP Handler Tests (with Database):**
- `tests/handler_employee_with_db_test.rs` - Employee handler HTTP tests (2 tests)
- `tests/handler_department_with_db_test.rs` - Department handler HTTP tests (4 tests)
- `tests/handler_salary_grade_with_db_test.rs` - Salary grade handler HTTP tests (4 tests)
- `tests/handler_test.rs` - Legacy handler tests
- `tests/handler_personnel_test.rs` - Personnel management handler tests

**Test Utilities:**
- `tests/common/mod.rs` - Shared database utilities and test helpers

### Coverage Statistics

- **Overall Coverage:** 31.00% (142/458 lines)
- **Department Handlers:** 50.00% (45/90 lines)
- **Salary Grade Handlers:** 52.63% (50/95 lines)  
- **Employee Handlers:** 20.73% (40/193 lines)

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test file
cargo test unit_models_test
cargo test db_employee_integration_test
cargo test handler_employee_with_db_test

# Run with output
cargo test -- --nocapture

# Run tests with coverage report
cargo tarpaulin --workspace --all-features --verbose --out lcov --engine llvm
```

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
lf11a_project_backend/
├── .github/
│   └── workflows/
│       └── ci.yml           # GitHub Actions CI/CD pipeline
├── src/
│   ├── main.rs              # Server setup, OpenAPI config, and routes
│   ├── lib.rs               # Library exports for tests
│   ├── handler.rs           # Basic/legacy handlers (health, users)
│   ├── handlers/            # Personnel management handlers (modular)
│   │   ├── mod.rs           # Module exports
│   │   ├── employee.rs      # Employee CRUD and assignment handlers
│   │   ├── department.rs    # Department management handlers
│   │   └── salary_grade.rs  # Salary grade management handlers
│   ├── models.rs            # Data structures with OpenAPI schemas
│   └── db.rs                # Database connection pool
├── tests/
│   ├── common/
│   │   └── mod.rs           # Shared test utilities and database helpers
│   ├── unit_models_test.rs             # Model unit tests (25 tests)
│   ├── handlers_module_test.rs         # Request/response validation (19 tests)
│   ├── integration_employee_test.rs    # Employee endpoint tests (7 tests)
│   ├── integration_department_test.rs  # Department endpoint tests (4 tests)
│   ├── integration_salary_grade_test.rs # Salary grade endpoint tests (4 tests)
│   ├── integration_handler_test.rs     # Legacy handler tests (5 tests)
│   ├── db_employee_integration_test.rs # Employee DB operations (13 tests)
│   ├── db_department_integration_test.rs # Department DB operations (9 tests)
│   ├── db_salary_grade_integration_test.rs # Salary grade DB operations (10 tests)
│   ├── handler_employee_with_db_test.rs # Employee HTTP+DB tests (2 tests)
│   ├── handler_department_with_db_test.rs # Department HTTP+DB tests (4 tests)
│   ├── handler_salary_grade_with_db_test.rs # Salary grade HTTP+DB tests (4 tests)
│   ├── direct_handler_test.rs          # Basic handler tests (3 tests)
│   ├── handler_test.rs                 # Legacy handler tests
│   ├── handler_personnel_test.rs       # Personnel handler tests
│   ├── models_test.rs                  # Model tests
│   └── db_test.rs                      # Database connection tests
├── Cargo.toml               # Dependencies and project configuration
├── Cargo.lock               # Locked dependency versions
├── docker-compose.yml       # MySQL Docker setup
├── init.sql                 # Database schema and seed data
├── lcov.info                # Code coverage report
├── .env                     # Environment variables (DATABASE_URL, HOST, PORT)
├── .env_example             # Example environment configuration
├── .gitignore               # Git ignore patterns
└── README.md                # This file
```

## Key Dependencies

- **actix-web** - Web framework
- **mysql** - MySQL database driver
- **serde** & **serde_json** - Serialization/deserialization
- **uuid** - UUID generation
- **dotenv** - Environment variable management
- **utoipa** - OpenAPI specification generation
- **utoipa-swagger-ui** - Swagger UI integration

## Environment Variables

The application is configured via a `.env` file in the project root:

```env
# Database Configuration
# Format: mysql://username:password@host:port/database
DATABASE_URL=mysql://user:userpassword@localhost:3307/mydb

# Server Configuration
HOST=127.0.0.1
PORT=8080
```

**Configuration Notes:**
- `DATABASE_URL`: Connection string for MySQL (matches docker-compose.yml settings)
- `HOST`: Server bind address (use `0.0.0.0` to accept external connections)
- `PORT`: Server port (defaults to 8080 if not set)

## Example Usage Scenarios

### Using Swagger UI (Recommended)

The easiest way to interact with the API is through the Swagger UI at `http://127.0.0.1:8080/docs/`

1. Open your browser to `http://127.0.0.1:8080/docs/`
2. Browse available endpoints organized by tags
3. Click on any endpoint to expand it
4. Click "Try it out" button
5. Fill in required parameters/request body
6. Click "Execute" to send the request
7. View the response below

### Using cURL (Command Line)

#### Scenario 1: Create a new employee and assign them to a department with a manager

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

#### Scenario 2: Promote an employee and change their salary grade

```bash
# Update role and salary grade
curl -X PUT http://127.0.0.1:8080/api/employees/{employee-id} \
  -H "Content-Type: application/json" \
  -d '{
    "role": "DeputyHead",
    "salary_grade_id": "550e8400-e29b-41d4-a716-446655440005"
  }'
```

#### Scenario 3: Create a new department with a department head

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
