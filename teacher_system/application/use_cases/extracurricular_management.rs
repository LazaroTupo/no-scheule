use crate::application::dto::ScheduleDTO;
use crate::domain::{
    models::Schedule,
    repositories::{ScheduleRepository, UserRepository},
    services::SchedulingService,
};

pub struct ExtracurricularManagementUseCase<'a> {
    scheduling_service: &'a dyn SchedulingService,
    user_repo: &'a dyn UserRepository,
}

impl<'a> ExtracurricularManagementUseCase<'a> {
    pub fn new(
        scheduling_service: &'a dyn SchedulingService,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        Self {
            scheduling_service,
            user_repo,
        }
    }

    pub async fn schedule_activity(
        &self,
        teacher_id: &str,
        activity: ScheduleDTO,
    ) -> Result<Schedule, String> {
        // Validar que el profesor existe
        let teacher = self.user_repo.get_user(teacher_id).await?;
        if teacher.is_none() {
            return Err("Profesor no encontrado".into());
        }

        // Convertir DTO a modelo
        let schedule = Schedule {
            id: format!("act_{}", uuid::Uuid::new_v4()),
            facility_id: activity.facility_id.unwrap_or_default(),
            day: activity.day,
            start_time: activity.start_time,
            end_time: activity.end_time,
            session_type: activity.session_type,
            location_detail: activity.location_detail,
        };

        // Programar actividad
        self.scheduling_service
            .validate_schedule(teacher_id, &schedule)
            .await?;

        Ok(schedule)
    }

    pub async fn suggest_available_times(
        &self,
        teacher_id: &str,
        duration_minutes: i32,
        preferred_days: Vec<Weekday>,
    ) -> Result<Vec<Schedule>, String> {
        self.scheduling_service
            .suggest_available_time(teacher_id, duration_minutes, preferred_days)
            .await
    }
}
