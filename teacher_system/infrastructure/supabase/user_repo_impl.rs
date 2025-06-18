use crate::domain::{models::User, repositories::UserRepository};
use async_trait::async_trait;
use supabase_rs::SupabaseClient;

pub struct SupabaseUserRepository {
    client: SupabaseClient,
}

impl SupabaseUserRepository {
    pub fn new(client: SupabaseClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl UserRepository for SupabaseUserRepository {
    async fn get_user(&self, id: &str) -> Result<Option<User>, String> {
        let response = self
            .client
            .from("users")
            .select("*")
            .eq("id", id)
            .execute()
            .await
            .map_err(|e| e.to_string())?;

        if response.status().is_success() {
            let users: Vec<User> = response.json().await.map_err(|e| e.to_string())?;
            Ok(users.into_iter().next())
        } else {
            Err("Error al obtener usuario".into())
        }
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
}
