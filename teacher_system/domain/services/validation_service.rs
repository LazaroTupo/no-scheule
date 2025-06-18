use crate::domain::models::{Schedule, User};
use async_trait::async_trait;

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
    course_repo: Box<dyn CourseRepository>,
    schedule_repo: Box<dyn ScheduleRepository>,
    user_repo: Box<dyn UserRepository>,
}

impl DefaultValidationService {
    pub fn new(
        course_repo: Box<dyn CourseRepository>,
        schedule_repo: Box<dyn ScheduleRepository>,
        user_repo: Box<dyn UserRepository>,
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
    ) -> Result<bool, String>;

    async fn validate_teacher_hours(
        &self,
        teacher_id: &str,
        additional_hours: i32,
    ) -> Result<bool, String>;
}
