use crate::domain::models::schedule::Schedule;
use async_trait::async_trait;

#[async_trait]
pub trait ScheduleRepository: Send + Sync {
    async fn create_schedule(&self, schedule: &Schedule) -> Result<(), String>;
    async fn update_schedule(&self, schedule: &Schedule) -> Result<(), String>;
    async fn get_schedule_by_id(&self, schedule_id: &str) -> Result<Option<Schedule>, String>;
    async fn get_all_schedules(&self) -> Result<Vec<Schedule>, String>;
    async fn get_schedules_by_course(&self, course_id: &str) -> Result<Option<Schedule>, String>;
    async fn get_schedules_by_course_name(
        &self,
        name_course: &str,
    ) -> Result<Vec<Schedule>, String>;
    async fn get_schedules_by_facility(&self, facility_id: &str) -> Result<Vec<Schedule>, String>;
    async fn get_schedules_by_facility_name(
        &self,
        name_facility: &str,
    ) -> Result<Vec<Schedule>, String>;
    async fn get_schedules_by_user(&self, user_id: &str) -> Result<Vec<Schedule>, String>;
    async fn get_schedules_by_weekday(&self, weekday: &str) -> Result<Vec<Schedule>, String>;
    async fn delete_schedule(&self, schedule_id: &str) -> Result<(), String>;
}
