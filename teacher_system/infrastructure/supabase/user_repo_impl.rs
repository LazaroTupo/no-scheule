use crate::domain::{models::user::User, repositories::user_repository::UserRepository};
use async_trait::async_trait;
use std::sync::Arc;
use supabase_rs::SupabaseClient;

#[derive(Clone)]
pub struct SupabaseUserRepository {
    client: Arc<SupabaseClient>,
}

impl SupabaseUserRepository {
    pub fn new(client: Arc<SupabaseClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl UserRepository for SupabaseUserRepository {
    async fn get_user(&self, id: &str) -> Result<Option<User>, String> {
        println!("Llamada a get_user con id: {}", id);
        Ok(None)
    }

    async fn get_teacher_details(&self, teacher_id: &str) -> Result<User, String> {
        let user = self
            .get_user(teacher_id)
            .await?
            .ok_or_else(|| "Profesor no encontrado".to_string())?;

        if !user.is_teacher() {
            return Err("El usuario no es un profesor".into());
        }

        Ok(user)
    }

    // Implementar otros métodos
    async fn get_students_in_course(&self, course_id: &str) -> Result<Vec<User>, String> {
        println!(
            "Llamada a get_students_in_course con course_id: {}",
            course_id
        );
        Ok(vec![])
    }
}
