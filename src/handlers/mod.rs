// Personnel handlers module - organized by domain

pub mod employee;
pub mod department;
pub mod salary_grade;

// Re-export all handlers for easy access
pub use employee::*;
pub use department::*;
pub use salary_grade::*;
