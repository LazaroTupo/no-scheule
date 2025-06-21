use crate::application::dto::schedule_dto::ScheduleDTO;
use crate::domain::{
    models::schedule::Schedule, repositories::schedule_repository::ScheduleRepository,
    services::validation_service::ValidationService,
};

pub struct ScheduleManagementUseCase<'a> {
    schedule_repo: &'a dyn ScheduleRepository,
    validation_service: &'a dyn ValidationService,
}

impl<'a> ScheduleManagementUseCase<'a> {
    pub fn new(
        schedule_repo: &'a dyn ScheduleRepository,
        validation_service: &'a dyn ValidationService,
    ) -> Self {
        Self {
            schedule_repo,
            validation_service,
        }
    }

    pub async fn set_course_schedule(
        &self,
        course_id: &str,
        schedule: ScheduleDTO,
    ) -> Result<Schedule, String> {
        // Convertir DTO a modelo de dominio
        let new_schedule = Schedule {
            id: format!("sch_{}", uuid::Uuid::new_v4()),
            facility_id: schedule.facility_id.unwrap_or_default(),
            day: schedule.day,
            start_time: schedule.start_time,
            end_time: schedule.end_time,
            session_type: schedule.session_type,
            location_detail: schedule.location_detail,
        };

        // Validar disponibilidad
        let is_teacher_available = self
            .validation_service
            .check_teacher_availability(course_id, &new_schedule)
            .await?;

        if !is_teacher_available {
            return Err("El profesor no está disponible en ese horario".into());
        }

        // Crear horario
        self.schedule_repo.create_schedule(&new_schedule).await?;

        Ok(new_schedule)
    }
}
