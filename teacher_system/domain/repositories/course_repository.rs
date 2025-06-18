use async_trait::async_trait;
use crate::domain::models::{Course, Schedule};

#[async_trait]
pub trait CourseRepository: Send + Sync {
    async fn create_course(&self, course: &Course) -> Result<(), String>;
    async fn update_course(&self, course: &Course) -> Result<(), String>;
    async fn get_course_by_id(&self, id: &str) -> Result<Option<Course>, String>;
    async fn get_teacher_courses(&self, teacher_id: &str) -> Result<Vec<Course>, String>;
    async fn check_schedule_conflict(
        &self, 
        teacher_id: &str, 
        schedule: &Schedule
    ) -> Result<bool, String>;
}