use crate::domain::models::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user_by_id(&self, user_id: &str) -> Result<Option<User>, String>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, String>;
    async fn get_all_users(&self) -> Result<Vec<User>, String>;
    async fn create_user(&self, user: &User) -> Result<(), String>;
    async fn update_user(&self, user: &User) -> Result<(), String>;
    async fn delete_user(&self, user_id: &str) -> Result<(), String>;
    async fn get_users_by_course(&self, course_id: &str) -> Result<Vec<User>, String>;
    async fn get_users_by_course_name(&self, name_course: &str) -> Result<Vec<User>, String>;
    async fn get_users_by_facility(&self, facility_id: &str) -> Result<Vec<User>, String>;
    async fn get_users_by_facility_name(&self, name_facility: &str) -> Result<Vec<User>, String>;
    async fn get_users_by_schedule(&self, schedule_id: &str) -> Result<Vec<User>, String>;
    async fn get_users_by_name(&self, name: &str) -> Result<Vec<User>, String>;
}
