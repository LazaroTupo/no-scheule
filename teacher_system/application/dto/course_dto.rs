use crate::domain::models::{Curriculum, SessionType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseDTO {
    pub code: String,
    pub name: String,
    pub section: i32,
    pub curriculum: Curriculum,
    pub capacity: i32,
    pub credits: i32,
    pub hours_per_week: i32,
    pub cycle: i32,
    pub semester: String,
    pub academic_year: i32,
    pub session_type: SessionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseResponseDTO {
    pub id: String,
    pub code: String,
    pub name: String,
    pub section: i32,
    pub schedule: ScheduleDTO,
    pub enrolled: i32,
    pub capacity: i32,
    pub available_spots: i32,
}
