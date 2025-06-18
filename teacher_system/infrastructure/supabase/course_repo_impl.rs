use crate::domain::{models::Course, repositories::CourseRepository};
use async_trait::async_trait;
use supabase_rs::SupabaseClient;

pub struct SupabaseCourseRepository {
    client: SupabaseClient,
}

impl SupabaseCourseRepository {
    pub fn new(client: SupabaseClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl CourseRepository for SupabaseCourseRepository {
    async fn create_course(&self, course: &Course) -> Result<(), String> {
        let response = self
            .client
            .from("courses")
            .insert(serde_json::json!({
                "id": course.id,
                "code": course.code,
                "name": course.name,
                // otros campos
            }))
            .execute()
            .await
            .map_err(|e| e.to_string())?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err("Error al crear curso".into())
        }
    }

    // Implementar otros métodos
}
