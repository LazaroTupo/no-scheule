use crate::domain::models::enums::{ContractType, StudentStatus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub code: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub faculty: String,
    pub program: String,
    pub specialty: String,
    pub role: String,
    pub student_status: Option<StudentStatus>,
    pub admission_date: Option<String>,
    pub contract_type: Option<ContractType>,
    pub max_hours_per_week: Option<i32>,
    pub hire_date: Option<String>,
    pub full_name: String,
}

impl User {
    pub fn is_teacher(&self) -> bool {
        self.contract_type.is_some()
    }
}
