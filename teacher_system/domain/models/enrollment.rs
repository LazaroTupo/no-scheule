use crate::domain::models::enums::EnrollmentStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enrollment {
    pub id: String,
    pub student_id: String,
    pub course_id: String,
    pub status: EnrollmentStatus,
}
