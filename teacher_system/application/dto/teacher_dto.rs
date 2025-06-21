use crate::application::dto::course_dto::CourseResponseDTO;
use crate::application::dto::schedule_dto::ScheduleResponseDTO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeacherDTO {
    pub id: String,
    pub code: String,
    pub name: String,
    pub email: String,
    pub faculty: String,
    pub max_hours_per_week: i32,
    pub current_hours: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeacherScheduleDTO {
    pub courses: Vec<CourseResponseDTO>,
    pub extracurricular: Vec<ScheduleResponseDTO>,
}
