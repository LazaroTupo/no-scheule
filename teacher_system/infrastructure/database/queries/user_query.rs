use crate::domain::{models::user::User, repositories::user_repository::UserRepository};
use crate::infrastructure::database::{
    conexion,
    entities::{course_schedules, courses, facilities, sea_orm_active_enums, users},
};
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
    Set,
};

#[derive(Clone)]
pub struct SupabaseUserRepository;
impl SupabaseUserRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl UserRepository for SupabaseUserRepository {
    async fn get_user(&self, id: &str) -> Result<Option<User>, String> {
        let db = conexion::get_conn();

        let user = users::Entity::find()
            .filter(users::Column::Id.eq(id))
            .one(db)
            .await
            .map_err(|e| e.to_string())?
            .map(|u| User {
                id: u.id,
                code: u.code,
                email: u.email,
                phone: u.phone,
                faculty: u.faculty,
                program: u.program.unwrap_or_default(),
                specialty: u.specialty,
                student_status: Some(sea_orm_active_enums::to_domain_student_status(
                    &u.student_status,
                )),
                admission_date: u.admission_date.map(|d| d.to_string()),
                contract_type: Some(u.contract_type),
                max_hours_per_week: u.max_hours_per_week,
                hire_date: u.hire_date.map(|d| d.to_string()),
                active: u.contract_type.is_some(),
            });

        Ok(user)
    }
    async fn get_user_by_id(&self, user_id: &str) -> Result<User, String>;
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
    async fn get_users_by_name(&self, day: &str) -> Result<Vec<User>, String>;
}
