// Personnel handlers module - organized by domain

pub mod department;
pub mod employee;
pub mod salary_grade;

// Re-export all handlers for easy access
pub use department::*;
pub use employee::*;
pub use salary_grade::*;
