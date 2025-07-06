use crate::domain::{
    models::enums::{SessionType, Weekday},
    models::schedule::Schedule,
    repositories::{course_repository::CourseRepository, schedule_repository::ScheduleRepository},
};
use chrono::NaiveTime;
use chrono::Utc;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::sync::Arc;

#[derive(Clone)]
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

impl DefaultSchedulingService {
    pub async fn suggest_available_time(&self, teacher_id: &str) -> Result<Vec<Schedule>, String> {
        // Get teacher's courses
        let teacher_courses = self.course_repo.get_courses_by_user(&teacher_id).await?;

        if teacher_courses.is_empty() {
            return Err("Teacher has no courses".to_string());
        }

        // Get busy schedules
        let mut busy_schedules = Vec::new();
        for course in &teacher_courses {
            match self.schedule_repo.get_schedules_by_course(&course.id).await {
                Ok(schedule) => busy_schedules.extend(schedule),
                Err(e) => return Err(e),
            }
        }

        // Create a list of all possible weekdays (Monday to Saturday)
        let all_days = vec![
            Weekday::Monday,
            Weekday::Tuesday,
            Weekday::Wednesday,
            Weekday::Thursday,
            Weekday::Friday,
            Weekday::Saturday,
        ];

        // Shuffle the days to get random order
        let mut rng = thread_rng();
        let mut shuffled_days = all_days.clone();
        shuffled_days.shuffle(&mut rng);

        // Work hours (8:00 to 22:00)
        let work_start = NaiveTime::from_hms_opt(8, 0, 0).unwrap();
        let work_end = NaiveTime::from_hms_opt(22, 0, 0).unwrap();

        // Try to find available slots for each course
        let mut suggested_schedules = Vec::new();

        for course in &teacher_courses {
            let hours_needed = course.hours_per_week;
            let duration = chrono::Duration::hours(hours_needed as i64);

            // Try each day in random order
            for &day in &shuffled_days {
                // Check if teacher already has a schedule on this day
                let has_schedule_on_day = busy_schedules.iter().any(|s| s.day == day);

                if has_schedule_on_day {
                    continue;
                }

                // Find available time slot for this day
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
                        created_at: Some(Utc::now().naive_utc().to_string()),
                        course_id: course.id.clone(),
                    };

                    // Check if the slot conflicts with any existing schedule
                    let is_available = !busy_schedules
                        .iter()
                        .any(|busy| busy.conflicts_with(&proposed_slot));

                    if is_available {
                        suggested_schedules.push(proposed_slot.clone());
                        // Mark this time as busy for subsequent checks
                        busy_schedules.push(proposed_slot.clone());
                        break; // Move to next course after finding a slot
                    }

                    // Move forward by 30 minutes
                    current_time = current_time + chrono::Duration::minutes(30);
                }

                if !suggested_schedules.is_empty()
                    && suggested_schedules.last().unwrap().course_id == course.id
                {
                    break; // Found a slot for this course, move to next
                }
            }
        }

        if suggested_schedules.len() < teacher_courses.len() {
            return Err("Could not find available slots for all courses".to_string());
        }

        Ok(suggested_schedules)
    }

    pub async fn validate_schedule(
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
