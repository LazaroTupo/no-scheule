use crate::domain::models::course::Course;
use async_trait::async_trait;

#[async_trait]
pub trait CourseRepository: Send + Sync {
    async fn create_course(&self, course: &Course) -> Result<(), String>;
    async fn update_course(&self, course: &Course) -> Result<(), String>;
    async fn get_course_by_id(&self, id: &str) -> Result<Course, String>;
    async fn get_all_courses(&self) -> Result<Vec<Course>, String>;
    async fn get_courses_by_user(&self, user_id: &str) -> Result<Vec<Course>, String>;
    async fn get_courses_by_facility(&self, facility_id: &str) -> Result<Vec<Course>, String>;
    async fn get_courses_by_facility_name(
        &self,
        name_facility: &str,
    ) -> Result<Vec<Course>, String>;
    async fn get_courses_by_schedule(&self, schedule_id: &str) -> Result<Course, String>;
    async fn delete_course(&self, id: &str) -> Result<(), String>;
}
