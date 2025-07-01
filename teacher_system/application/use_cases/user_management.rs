// src/application/usecases/user_management_usecase.rs

use crate::domain::{models::user::User, repositories::user_repository::UserRepository};
use async_trait::async_trait;

#[async_trait]
pub trait UserManagementUseCase {
    async fn get_all(&self) -> Result<Vec<User>, String>;
    async fn get_by_id(&self, id: &str) -> Result<User, String>;
    async fn get_by_email(&self, email: &str) -> Result<User, String>;
    async fn create(&self, user: User) -> Result<User, String>;
    async fn update(&self, user: &User) -> Result<(), String>;
    async fn delete(&self, id: &str) -> Result<(), String>;
    async fn get_by_name(&self, name: &str) -> Result<Vec<User>, String>;
    async fn get_by_course(&self, course_id: &str) -> Result<Vec<User>, String>;
    async fn get_by_facility(&self, facility_id: &str) -> Result<Vec<User>, String>;
}
pub struct UserManagementUseCaseImpl {
    user_repo: Box<dyn UserRepository + Send + Sync>,
}

impl UserManagementUseCaseImpl {
    pub fn new(user_repo: Box<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repo }
    }
}

#[async_trait]
impl UserManagementUseCase for UserManagementUseCaseImpl {
    async fn get_all(&self) -> Result<Vec<User>, String> {
        self.user_repo.get_all_users().await
    }

    async fn get_by_id(&self, id: &str) -> Result<User, String> {
        self.user_repo
            .get_user_by_id(id)
            .await?
            .ok_or_else(|| "User not found".to_string())
    }

    async fn get_by_email(&self, email: &str) -> Result<User, String> {
        self.user_repo
            .get_user_by_email(email)
            .await?
            .ok_or_else(|| "User not found".to_string())
    }

    async fn create(&self, user: User) -> Result<User, String> {
        self.user_repo.create_user(&user).await?;
        Ok(user)
    }

    async fn update(&self, user: &User) -> Result<(), String> {
        self.user_repo.update_user(user).await
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        self.user_repo.delete_user(id).await
    }

    async fn get_by_name(&self, name: &str) -> Result<Vec<User>, String> {
        self.user_repo.get_users_by_name(name).await
    }

    async fn get_by_course(&self, course_id: &str) -> Result<Vec<User>, String> {
        self.user_repo.get_users_by_course(course_id).await
    }

    async fn get_by_facility(&self, facility_id: &str) -> Result<Vec<User>, String> {
        self.user_repo.get_users_by_facility(facility_id).await
    }
}
