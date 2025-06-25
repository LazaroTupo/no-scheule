use crate::domain::{
    models::schedule::{Schedule, SessionType, Weekday},
    repositories::{
        course_repository::CourseRepository, schedule_repository::ScheduleRepository,
        user_repository::UserRepository,
    },
};
use async_trait::async_trait;
use chrono::NaiveTime;
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
    course_repo: Arc<dyn CourseRepository + Send + Sync>,
    schedule_repo: Arc<dyn ScheduleRepository + Send + Sync>,
    user_repo: Arc<dyn UserRepository + Send + Sync>,
}

impl DefaultSchedulingService {
    pub fn new(
        course_repo: Arc<dyn CourseRepository + Send + Sync>,
        schedule_repo: Arc<dyn ScheduleRepository + Send + Sync>,
        user_repo: Arc<dyn UserRepository + Send + Sync>,
    ) -> Self {
        Self {
            course_repo,
            schedule_repo,
            user_repo,
        }
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
        // Obtener todos los cursos del profesor
        let teacher_courses = self.course_repo.get_teacher_courses(teacher_id).await?;

        // Obtener todos los horarios de esos cursos
        let mut busy_schedules = Vec::new();
        for course in teacher_courses {
            if let Ok(schedule) = self.course_repo.get_course_schedule(&course.id).await? {
                busy_schedules.push(schedule);
            }
        }

        Ok(vec![])
    }

    async fn validate_schedule(
        &self,
        teacher_id: &str,
        schedule: &Schedule,
    ) -> Result<bool, String> {
        // Implementar lógica para validar si el horario pruesto
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
