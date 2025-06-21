use crate::domain::models::schedule::{Schedule, Weekday};
use async_trait::async_trait;

#[async_trait]
pub trait ScheduleRepository: Send + Sync {
    async fn create_schedule(&self, schedule: &Schedule) -> Result<(), String>;
    async fn update_schedule(&self, schedule: &Schedule) -> Result<(), String>;
    async fn get_schedule(&self, id: &str) -> Result<Option<Schedule>, String>;
    async fn get_teacher_schedules(&self, teacher_id: &str) -> Result<Vec<Schedule>, String>;
    async fn get_facility_schedules(&self, facility_id: &str) -> Result<Vec<Schedule>, String>;
    async fn find_available_schedules(
        &self,
        duration_minutes: i32,
        preferred_days: Vec<Weekday>,
    ) -> Result<Vec<Schedule>, String>;
}
