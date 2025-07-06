use crate::domain::{
    models::{enums::Weekday, schedule::Schedule},
    repositories::schedule_repository::ScheduleRepository,
    services::{
        scheduling_service::DefaultSchedulingService, validation_service::DefaultValidationService,
    },
};
use async_trait::async_trait;

#[async_trait]
pub trait ScheduleManagementUseCase {
    async fn get_all(&self) -> Result<Vec<Schedule>, String>;
    async fn get_by_id(&self, id: &str) -> Result<Schedule, String>;
    async fn create(&self, schedule: Schedule) -> Result<(), String>;
    async fn update(&self, schedule: &Schedule) -> Result<(), String>;
    async fn delete(&self, id: &str) -> Result<(), String>;
    async fn suggest_available_times(
        &self,
        teacher_id: &str,
        duration_minutes: i32,
        preferred_days: Vec<Weekday>,
    ) -> Result<Vec<Schedule>, String>;
}
pub struct ScheduleManagementUseCaseImpl {
    schedule_repo: Box<dyn ScheduleRepository + Send + Sync>,
    validation_service: DefaultValidationService,
    scheduling_service: DefaultSchedulingService,
}

impl ScheduleManagementUseCaseImpl {
    pub fn new(
        schedule_repo: Box<dyn ScheduleRepository + Send + Sync>,
        validation_service: DefaultValidationService,
        scheduling_service: DefaultSchedulingService,
    ) -> Self {
        Self {
            schedule_repo,
            validation_service,
            scheduling_service,
        }
    }
}

#[async_trait]
impl ScheduleManagementUseCase for ScheduleManagementUseCaseImpl {
    async fn get_all(&self) -> Result<Vec<Schedule>, String> {
        self.schedule_repo.get_all_schedules().await
    }

    async fn get_by_id(&self, id: &str) -> Result<Schedule, String> {
        self.schedule_repo
            .get_schedule_by_id(id)
            .await?
            .ok_or_else(|| "Schedule not found".to_string())
    }

    async fn create(&self, schedule: Schedule) -> Result<(), String> {
        let is_available = self
            .validation_service
            .check_facility_availability(&schedule.facility_id, &schedule)
            .await?;

        if !is_available {
            return Err("Facility not available at requested time".to_string());
        }

        self.schedule_repo.create_schedule(&schedule).await
    }

    async fn update(&self, schedule: &Schedule) -> Result<(), String> {
        let is_available = self
            .validation_service
            .check_facility_availability(&schedule.facility_id, schedule)
            .await?;

        if !is_available {
            return Err("Facility not available at requested time".to_string());
        }

        self.schedule_repo.update_schedule(schedule).await
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        self.schedule_repo.delete_schedule(id).await
    }

    async fn suggest_available_times(
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
