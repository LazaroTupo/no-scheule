use crate::domain::models::schedule::Schedule;
use crate::domain::repositories::{
    course_repository::CourseRepository, schedule_repository::ScheduleRepository,
    user_repository::UserRepository,
};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait ValidationService: Send + Sync {
    async fn check_teacher_availability(
        &self,
        teacher_id: &str,
        schedule: &Schedule,
    ) -> Result<bool, String>;

    async fn check_facility_availability(
        &self,
        facility_id: &str,
        schedule: &Schedule,
    ) -> Result<bool, String>;

    async fn validate_teacher_hours(
        &self,
        teacher_id: &str,
        additional_hours: i32,
    ) -> Result<bool, String>;
}

pub struct DefaultValidationService {
    course_repo: Arc<dyn CourseRepository>,
    schedule_repo: Arc<dyn ScheduleRepository>,
    user_repo: Arc<dyn UserRepository>,
}

impl DefaultValidationService {
    pub fn new(
        course_repo: Arc<dyn CourseRepository>,
        schedule_repo: Arc<dyn ScheduleRepository>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            course_repo,
            schedule_repo,
            user_repo,
        }
    }
}

#[async_trait]
impl ValidationService for DefaultValidationService {
    async fn check_teacher_availability(
        &self,
        teacher_id: &str,
        schedule: &Schedule,
    ) -> Result<bool, String> {
        let teacher_schedules = self.schedule_repo.get_teacher_schedules(teacher_id).await?;
        let has_conflict = teacher_schedules.iter().any(|s| s.conflicts_with(schedule));
        Ok(!has_conflict)
    }

    async fn check_facility_availability(
        &self,
        facility_id: &str,
        schedule: &Schedule,
    ) -> Result<bool, String> {
        // Implement logic to check if the facility is available for the given schedule

        let facility_schedules = self
            .schedule_repo
            .get_facility_schedules(facility_id)
            .await?;
        let has_conflict = facility_schedules
            .iter()
            .any(|s| s.conflicts_with(schedule));
        Ok(!has_conflict)
    }

    async fn validate_teacher_hours(
        &self,
        teacher_id: &str,
        additional_hours: i32,
    ) -> Result<bool, String> {
        // Implement logic to validate if the teacher can take on additional hours

        // let teacher = self.user_repo.get_teacher_details(teacher_id).await?;
        // if let Some(teacher) = teacher {
        //     let total_hours = teacher.total_hours + additional_hours;
        //     if total_hours <= teacher.max_hours {
        //         return Ok(true);
        //     }
        // }
        // Err("Teacher cannot take on additional hours".to_string())
        println!(
            "Validando si el profesor {} puede tomar {} horas adicionales.",
            teacher_id, additional_hours
        );
        Ok(true)
    }
}
