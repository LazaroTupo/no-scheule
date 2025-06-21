use crate::domain::models::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user(&self, id: &str) -> Result<Option<User>, String>;
    async fn get_teacher_details(&self, teacher_id: &str) -> Result<User, String>;
    async fn get_students_in_course(&self, course_id: &str) -> Result<Vec<User>, String>;
}
