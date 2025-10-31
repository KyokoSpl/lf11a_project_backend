-- Drop existing tables if they exist (for clean restart)
DROP TABLE IF EXISTS employees;
DROP TABLE IF EXISTS salary_grades;
DROP TABLE IF EXISTS departments;

-- Create departments table
CREATE TABLE departments (
  id CHAR(36) PRIMARY KEY,
  name VARCHAR(255) NOT NULL UNIQUE,
  head_id CHAR(36) NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NULL ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB;

-- Create salary_grades table
CREATE TABLE salary_grades (
  id CHAR(36) PRIMARY KEY,
  code VARCHAR(50) NOT NULL UNIQUE,
  base_salary DECIMAL(12,2) NOT NULL,
  description TEXT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB;

-- Create employees table
CREATE TABLE employees (
  id CHAR(36) PRIMARY KEY,
  first_name VARCHAR(100) NOT NULL,
  last_name VARCHAR(100) NOT NULL,
  email VARCHAR(255) NOT NULL UNIQUE,
  department_id CHAR(36) NULL,
  salary_grade_id CHAR(36) NULL,
  manager_id CHAR(36) NULL,
  role ENUM('Admin','DepartmentHead','DeputyHead','Employee') NOT NULL DEFAULT 'Employee',
  hire_date DATE NULL,
  active BOOLEAN NOT NULL DEFAULT TRUE,
  deleted_at TIMESTAMP NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NULL ON UPDATE CURRENT_TIMESTAMP,
  CONSTRAINT fk_emp_department FOREIGN KEY (department_id) REFERENCES departments(id) ON DELETE SET NULL,
  CONSTRAINT fk_emp_salary FOREIGN KEY (salary_grade_id) REFERENCES salary_grades(id) ON DELETE SET NULL,
  CONSTRAINT fk_emp_manager FOREIGN KEY (manager_id) REFERENCES employees(id) ON DELETE SET NULL
) ENGINE=InnoDB;

-- Create indexes
CREATE INDEX idx_emp_department ON employees(department_id);
CREATE INDEX idx_emp_manager ON employees(manager_id);
CREATE INDEX idx_emp_salary ON employees(salary_grade_id);
CREATE INDEX idx_emp_active ON employees(active);

-- Insert sample salary grades
INSERT INTO salary_grades (id, code, base_salary, description) VALUES
  ('550e8400-e29b-41d4-a716-446655440001', 'E1', 45000.00, 'Entry Level'),
  ('550e8400-e29b-41d4-a716-446655440002', 'E2', 55000.00, 'Junior Level'),
  ('550e8400-e29b-41d4-a716-446655440003', 'E3', 70000.00, 'Mid Level'),
  ('550e8400-e29b-41d4-a716-446655440004', 'E4', 90000.00, 'Senior Level'),
  ('550e8400-e29b-41d4-a716-446655440005', 'M1', 110000.00, 'Manager Level'),
  ('550e8400-e29b-41d4-a716-446655440006', 'M2', 140000.00, 'Senior Manager Level'),
  ('550e8400-e29b-41d4-a716-446655440007', 'D1', 180000.00, 'Director Level');

-- Insert sample departments
INSERT INTO departments (id, name, head_id) VALUES
  ('650e8400-e29b-41d4-a716-446655440001', 'Engineering', NULL),
  ('650e8400-e29b-41d4-a716-446655440002', 'Human Resources', NULL),
  ('650e8400-e29b-41d4-a716-446655440003', 'Marketing', NULL),
  ('650e8400-e29b-41d4-a716-446655440004', 'Sales', NULL),
  ('650e8400-e29b-41d4-a716-446655440005', 'Finance', NULL),
  ('650e8400-e29b-41d4-a716-446655440006', 'IT Operations', NULL);

-- Insert sample employees (Admin and Department Heads first)
INSERT INTO employees (id, first_name, last_name, email, department_id, salary_grade_id, manager_id, role, hire_date, active) VALUES
  -- Admin
  ('750e8400-e29b-41d4-a716-446655440001', 'Alice', 'Johnson', 'alice.johnson@company.com', NULL, '550e8400-e29b-41d4-a716-446655440007', NULL, 'Admin', '2020-01-15', TRUE),
  
  -- Department Heads
  ('750e8400-e29b-41d4-a716-446655440002', 'Bob', 'Smith', 'bob.smith@company.com', '650e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440006', '750e8400-e29b-41d4-a716-446655440001', 'DepartmentHead', '2020-03-01', TRUE),
  ('750e8400-e29b-41d4-a716-446655440003', 'Carol', 'Williams', 'carol.williams@company.com', '650e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440006', '750e8400-e29b-41d4-a716-446655440001', 'DepartmentHead', '2020-04-10', TRUE),
  ('750e8400-e29b-41d4-a716-446655440004', 'David', 'Brown', 'david.brown@company.com', '650e8400-e29b-41d4-a716-446655440003', '550e8400-e29b-41d4-a716-446655440006', '750e8400-e29b-41d4-a716-446655440001', 'DepartmentHead', '2020-05-20', TRUE),
  ('750e8400-e29b-41d4-a716-446655440005', 'Eva', 'Davis', 'eva.davis@company.com', '650e8400-e29b-41d4-a716-446655440004', '550e8400-e29b-41d4-a716-446655440006', '750e8400-e29b-41d4-a716-446655440001', 'DepartmentHead', '2020-06-15', TRUE),
  ('750e8400-e29b-41d4-a716-446655440006', 'Frank', 'Miller', 'frank.miller@company.com', '650e8400-e29b-41d4-a716-446655440005', '550e8400-e29b-41d4-a716-446655440006', '750e8400-e29b-41d4-a716-446655440001', 'DepartmentHead', '2020-07-01', TRUE),
  
  -- Deputy Heads
  ('750e8400-e29b-41d4-a716-446655440007', 'Grace', 'Wilson', 'grace.wilson@company.com', '650e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440005', '750e8400-e29b-41d4-a716-446655440002', 'DeputyHead', '2021-01-10', TRUE),
  ('750e8400-e29b-41d4-a716-446655440008', 'Henry', 'Moore', 'henry.moore@company.com', '650e8400-e29b-41d4-a716-446655440004', '550e8400-e29b-41d4-a716-446655440005', '750e8400-e29b-41d4-a716-446655440005', 'DeputyHead', '2021-02-15', TRUE),
  
  -- Regular Employees - Engineering
  ('750e8400-e29b-41d4-a716-446655440009', 'Iris', 'Taylor', 'iris.taylor@company.com', '650e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440004', '750e8400-e29b-41d4-a716-446655440002', 'Employee', '2021-03-01', TRUE),
  ('750e8400-e29b-41d4-a716-446655440010', 'Jack', 'Anderson', 'jack.anderson@company.com', '650e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440003', '750e8400-e29b-41d4-a716-446655440002', 'Employee', '2021-06-15', TRUE),
  ('750e8400-e29b-41d4-a716-446655440011', 'Kate', 'Thomas', 'kate.thomas@company.com', '650e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440002', '750e8400-e29b-41d4-a716-446655440007', 'Employee', '2022-01-10', TRUE),
  ('750e8400-e29b-41d4-a716-446655440012', 'Liam', 'Jackson', 'liam.jackson@company.com', '650e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440001', '750e8400-e29b-41d4-a716-446655440007', 'Employee', '2023-03-20', TRUE),
  
  -- Regular Employees - HR
  ('750e8400-e29b-41d4-a716-446655440013', 'Mia', 'White', 'mia.white@company.com', '650e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440003', '750e8400-e29b-41d4-a716-446655440003', 'Employee', '2021-08-01', TRUE),
  ('750e8400-e29b-41d4-a716-446655440014', 'Noah', 'Harris', 'noah.harris@company.com', '650e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440002', '750e8400-e29b-41d4-a716-446655440003', 'Employee', '2022-04-15', TRUE),
  
  -- Regular Employees - Marketing
  ('750e8400-e29b-41d4-a716-446655440015', 'Olivia', 'Martin', 'olivia.martin@company.com', '650e8400-e29b-41d4-a716-446655440003', '550e8400-e29b-41d4-a716-446655440003', '750e8400-e29b-41d4-a716-446655440004', 'Employee', '2021-09-10', TRUE),
  ('750e8400-e29b-41d4-a716-446655440016', 'Paul', 'Thompson', 'paul.thompson@company.com', '650e8400-e29b-41d4-a716-446655440003', '550e8400-e29b-41d4-a716-446655440002', '750e8400-e29b-41d4-a716-446655440004', 'Employee', '2022-07-01', TRUE),
  
  -- Regular Employees - Sales
  ('750e8400-e29b-41d4-a716-446655440017', 'Quinn', 'Garcia', 'quinn.garcia@company.com', '650e8400-e29b-41d4-a716-446655440004', '550e8400-e29b-41d4-a716-446655440004', '750e8400-e29b-41d4-a716-446655440005', 'Employee', '2021-10-05', TRUE),
  ('750e8400-e29b-41d4-a716-446655440018', 'Rachel', 'Martinez', 'rachel.martinez@company.com', '650e8400-e29b-41d4-a716-446655440004', '550e8400-e29b-41d4-a716-446655440003', '750e8400-e29b-41d4-a716-446655440008', 'Employee', '2022-02-20', TRUE),
  ('750e8400-e29b-41d4-a716-446655440019', 'Sam', 'Robinson', 'sam.robinson@company.com', '650e8400-e29b-41d4-a716-446655440004', '550e8400-e29b-41d4-a716-446655440002', '750e8400-e29b-41d4-a716-446655440008', 'Employee', '2023-01-15', TRUE),
  
  -- Regular Employees - Finance
  ('750e8400-e29b-41d4-a716-446655440020', 'Tara', 'Clark', 'tara.clark@company.com', '650e8400-e29b-41d4-a716-446655440005', '550e8400-e29b-41d4-a716-446655440004', '750e8400-e29b-41d4-a716-446655440006', 'Employee', '2021-11-01', TRUE),
  ('750e8400-e29b-41d4-a716-446655440021', 'Uma', 'Rodriguez', 'uma.rodriguez@company.com', '650e8400-e29b-41d4-a716-446655440005', '550e8400-e29b-41d4-a716-446655440003', '750e8400-e29b-41d4-a716-446655440006', 'Employee', '2022-05-10', TRUE),
  
  -- Regular Employees - IT Operations
  ('750e8400-e29b-41d4-a716-446655440022', 'Victor', 'Lewis', 'victor.lewis@company.com', '650e8400-e29b-41d4-a716-446655440006', '550e8400-e29b-41d4-a716-446655440003', NULL, 'Employee', '2022-08-15', TRUE),
  ('750e8400-e29b-41d4-a716-446655440023', 'Wendy', 'Lee', 'wendy.lee@company.com', '650e8400-e29b-41d4-a716-446655440006', '550e8400-e29b-41d4-a716-446655440002', NULL, 'Employee', '2023-02-01', TRUE),
  
  -- Inactive employee (soft deleted)
  ('750e8400-e29b-41d4-a716-446655440024', 'Xavier', 'Walker', 'xavier.walker@company.com', '650e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440002', '750e8400-e29b-41d4-a716-446655440002', 'Employee', '2021-04-01', FALSE);

-- Update department heads
UPDATE departments SET head_id = '750e8400-e29b-41d4-a716-446655440002' WHERE id = '650e8400-e29b-41d4-a716-446655440001';
UPDATE departments SET head_id = '750e8400-e29b-41d4-a716-446655440003' WHERE id = '650e8400-e29b-41d4-a716-446655440002';
UPDATE departments SET head_id = '750e8400-e29b-41d4-a716-446655440004' WHERE id = '650e8400-e29b-41d4-a716-446655440003';
UPDATE departments SET head_id = '750e8400-e29b-41d4-a716-446655440005' WHERE id = '650e8400-e29b-41d4-a716-446655440004';
UPDATE departments SET head_id = '750e8400-e29b-41d4-a716-446655440006' WHERE id = '650e8400-e29b-41d4-a716-446655440005';
