use crate::domain::models::Course;
use async_trait::async_trait;

#[async_trait]
pub trait CourseRepository: Send + Sync {
    async fn create_course(&self, course: &Course) -> Result<(), String>;
    async fn update_course(&self, course: &Course) -> Result<(), String>;
    async fn get_course_by_id(&self, id: &str) -> Result<Option<Course>, String>;
    async fn get_teacher_courses(&self, teacher_id: &str) -> Result<Vec<Course>, String>;
}
