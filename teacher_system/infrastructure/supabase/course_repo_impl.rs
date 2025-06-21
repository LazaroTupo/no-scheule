use crate::domain::{models::course::Course, repositories::course_repository::CourseRepository};
use async_trait::async_trait;
use std::sync::Arc;
use supabase_rs::SupabaseClient;

#[derive(Clone)]
pub struct SupabaseCourseRepository {
    client: Arc<SupabaseClient>,
}

impl SupabaseCourseRepository {
    pub fn new(client: Arc<SupabaseClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl CourseRepository for SupabaseCourseRepository {
    async fn create_course(&self, course: &Course) -> Result<(), String> {
        println!("Creando curso: {:?}", course);
        Ok(())
    }

    async fn update_course(&self, course: &Course) -> Result<(), String> {
        // implementar la lógica para actualizar un curso
        println!("Actualizando curso: {:?}", course);
        Ok(())
    }
    async fn get_course_by_id(&self, id: &str) -> Result<Option<Course>, String> {
        // implementar la lógica para obtener un curso por ID
        println!("Obteniendo curso por id: {}", id);
        Ok(None)
    }
    async fn get_teacher_courses(&self, teacher_id: &str) -> Result<Vec<Course>, String> {
        // implementar la lógica para obtener cursos de un profesor
        println!("Obteniendo cursos para el profesor con id: {}", teacher_id);
        Ok(vec![])
    }
}
