use crate::domain::{
    models::enums::{SessionType, Weekday},
    models::schedule::Schedule,
    repositories::{course_repository::CourseRepository, schedule_repository::ScheduleRepository},
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
}

impl DefaultSchedulingService {
    pub fn new(
        course_repo: Arc<dyn CourseRepository + Send + Sync>,
        schedule_repo: Arc<dyn ScheduleRepository + Send + Sync>,
    ) -> Self {
        Self {
            course_repo,
            schedule_repo,
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
        let teacher_courses = self.course_repo.get_courses_by_user(&teacher_id).await?;

        let mut busy_schedules = Vec::new();
        for course in teacher_courses {
            match self.schedule_repo.get_schedules_by_course(&course.id).await {
                Ok(schedule) => busy_schedules.extend(schedule),
                Err(e) => return Err(e),
            }
        }

        let work_start = NaiveTime::from_hms_opt(8, 0, 0).unwrap();
        let work_end = NaiveTime::from_hms_opt(22, 0, 0).unwrap();
        let duration = chrono::Duration::minutes(duration_minutes as i64);
        let interval = chrono::Duration::minutes(30);

        let days_to_check = if preferred_days.is_empty() {
            vec![
                Weekday::Monday,
                Weekday::Tuesday,
                Weekday::Wednesday,
                Weekday::Thursday,
                Weekday::Friday,
                Weekday::Saturday,
                Weekday::Sunday,
            ]
        } else {
            preferred_days
        };

        let mut available_slots = Vec::new();

        for day in days_to_check {
            let mut current_time = work_start;

            while current_time + duration <= work_end {
                let slot_end = current_time + duration;
                let proposed_slot = Schedule {
                    id: "temp".to_string(),
                    facility_id: "".to_string(),
                    day: day.clone(),
                    start_time: current_time,
                    end_time: slot_end,
                    session_type: SessionType::Theory,
                    location_detail: None,
                    created_at: None,
                    course_id: "temp_course".to_string(),
                };

                let is_available = !busy_schedules
                    .iter()
                    .any(|busy| busy.conflicts_with(&proposed_slot));

                if is_available {
                    available_slots.push(proposed_slot);
                }

                current_time = current_time + interval;
            }
        }

        Ok(available_slots)
    }

    async fn validate_schedule(
        &self,
        teacher_id: &str,
        schedule: &Schedule,
    ) -> Result<bool, String> {
        let teacher_courses = self.course_repo.get_courses_by_user(&teacher_id).await?;

        for course in teacher_courses {
            let existing_schedule = self
                .schedule_repo
                .get_schedules_by_course(&course.id)
                .await?
                .iter()
                .any(|s| s.conflicts_with(schedule));

            if existing_schedule {
                return Ok(false);
            }
        }

        Ok(true)
    }
}
