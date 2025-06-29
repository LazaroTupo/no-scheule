use crate::domain::models::facilitie::Facility;
use async_trait::async_trait;

#[async_trait]
pub trait FacilityRepository: Send + Sync {
    async fn create_facility(&self, facility: &Facility) -> Result<(), String>;
    async fn update_facility(&self, facility: &Facility) -> Result<(), String>;
    async fn get_facility_by_id(&self, id: &str) -> Result<Facility, String>;
    async fn get_all_facilities(&self) -> Result<Vec<Facility>, String>;
    async fn get_facilities_by_course(&self, course_id: &str) -> Result<Facility, String>;
    async fn get_facilities_name_course(&self, name_course: &str) -> Result<Vec<Facility>, String>;
    async fn get_facilities_by_schedule(&self, schedule_id: &str) -> Result<Facility, String>;
    async fn get_facilities_by_user(&self, user_id: &str) -> Result<Vec<Facility>, String>;
    async fn delete_facility(&self, id: &str) -> Result<(), String>;
}
