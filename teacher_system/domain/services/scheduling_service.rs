use crate::domain::models::schedule::{Schedule, Weekday};
use crate::domain::repositories::course_repository::CourseRepository;
use async_trait::async_trait;
use std::sync::Arc;

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
    course_repo: Arc<dyn CourseRepository>,
}

impl DefaultSchedulingService {
    pub fn new(course_repo: Arc<dyn CourseRepository>) -> Self {
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
    ) -> Result<bool, String> {
        // Implementar lógica para validar si el horario propuesto
        // no entra en conflicto con otros cursos del profesor

        // let courses = self.course_repo.get_courses_by_teacher(teacher_id).await?;
        // for course in courses {
        //     if course.schedule.conflicts_with(schedule) {
        //         return Ok(false);
        //     }
        // }
        Ok(true)
    }
}
