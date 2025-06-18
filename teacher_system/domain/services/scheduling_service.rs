use crate::domain::models::{Course, Schedule};
use async_trait::async_trait;

#[async_trait]
pub trait SchedulingService: Send + Sync {
    async fn suggest_available_time(
        &self,
        teacher_id: &str,
        duration_minutes: i32,
        preferred_days: Vec<Weekday>,
    ) -> Result<Vec<Schedule>, String>;

    async fn validate_schedule(
        &self,
        teacher_id: &str,
        schedule: &Schedule,
    ) -> Result<bool, String>;
}

pub struct DefaultSchedulingService {
    course_repo: Box<dyn CourseRepository>,
}

impl DefaultSchedulingService {
    pub fn new(course_repo: Box<dyn CourseRepository>) -> Self {
        Self { course_repo }
    }
}

#[async_trait]
impl SchedulingService for DefaultSchedulingService {
    async fn suggest_available_time(
        &self,
        teacher_id: &str,
        duration_minutes: i32,
        preferred_days: Vec<Weekday>,
    ) -> Result<Vec<Schedule>, String> {
        // Implementar lógica para sugerir horarios disponibles
        // basado en los cursos existentes del profesor
        Ok(vec![])
    }

    async fn validate_schedule(
        &self,
        teacher_id: &str,
        schedule: &Schedule,
    ) -> Result<bool, String>;
}
