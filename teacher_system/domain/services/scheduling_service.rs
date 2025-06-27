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

        // Definir el rango horario laboral típico (8am a 10pm)
        let work_start = NaiveTime::from_hms_opt(8, 0, 0).unwrap();
        let work_end = NaiveTime::from_hms_opt(22, 0, 0).unwrap();
        
        // Convertir minutos a duración
        let duration = chrono::Duration::minutes(duration_minutes as i64);
        
        // Generar posibles horarios
        let mut available_slots = Vec::new();
        
        // Considerar días preferidos o todos los días laborales si no hay preferencia
        let days_to_check = if preferred_days.is_empty() {
            vec![
                Weekday::Lunes,
                Weekday::Martes,
                Weekday::MIercoles,
                Weekday::Jueves,
                Weekday::Viernes,
                Weekday::Sabado,
            ]
        } else {
            preferred_days
        };

        // Intervalo de búsqueda (30 minutos)
        let interval = chrono::Duration::minutes(30);

        for day in days_to_check {
            let mut current_time = work_start;
            
            while current_time + duration <= work_end {
                let slot_end = current_time + duration;
                let proposed_slot = Schedule {
                    id: "temp".to_string(),
                    facility_id: "".to_string(),
                    day,
                    start_time: current_time,
                    end_time: slot_end,
                    session_type: SessionType::Theory,
                    location_detail: None,
                };
                
                // Verificar si el slot está disponible
                let is_available = !busy_schedules.iter().any(|busy| busy.conflicts_with(&proposed_slot));
                
                if is_available {
                    available_slots.push(proposed_slot);
                }
                
                // Avanzar al siguiente intervalo
                current_time = current_time + interval;
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
