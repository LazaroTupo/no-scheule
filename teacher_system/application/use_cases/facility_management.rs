// src/application/usecases/facility_management_usecase.rs

use crate::domain::{
    models::facilitie::Facility, repositories::facility_repository::FacilityRepository,
};
use async_trait::async_trait;

#[async_trait]
pub trait FacilityManagementUseCase {
    async fn get_all(&self) -> Result<Vec<Facility>, String>;
    async fn get_by_id(&self, id: &str) -> Result<Facility, String>;
    async fn create(&self, facility: Facility) -> Result<(), String>;
    async fn update(&self, facility: &Facility) -> Result<(), String>;
    async fn delete(&self, id: &str) -> Result<(), String>;
    async fn get_by_course(&self, course_id: &str) -> Result<Facility, String>;
    async fn get_by_name_course(&self, name_course: &str) -> Result<Vec<Facility>, String>;
    async fn get_by_schedule(&self, schedule_id: &str) -> Result<Facility, String>;
    async fn get_by_user(&self, user_id: &str) -> Result<Vec<Facility>, String>;
}

pub struct FacilityManagementUseCaseImpl {
    facility_repo: Box<dyn FacilityRepository + Send + Sync>,
}

impl FacilityManagementUseCaseImpl {
    pub fn new(facility_repo: Box<dyn FacilityRepository + Send + Sync>) -> Self {
        Self { facility_repo }
    }
}

#[async_trait]
impl FacilityManagementUseCase for FacilityManagementUseCaseImpl {
    async fn get_all(&self) -> Result<Vec<Facility>, String> {
        self.facility_repo.get_all_facilities().await
    }

    async fn get_by_id(&self, id: &str) -> Result<Facility, String> {
        self.facility_repo.get_facility_by_id(id).await
    }

    async fn create(&self, facility: Facility) -> Result<(), String> {
        self.facility_repo.create_facility(&facility).await
    }

    async fn update(&self, facility: &Facility) -> Result<(), String> {
        self.facility_repo.update_facility(facility).await
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        self.facility_repo.delete_facility(id).await
    }

    async fn get_by_course(&self, course_id: &str) -> Result<Facility, String> {
        self.facility_repo.get_facilities_by_course(course_id).await
    }

    async fn get_by_name_course(&self, name_course: &str) -> Result<Vec<Facility>, String> {
        self.facility_repo
            .get_facilities_name_course(name_course)
            .await
    }

    async fn get_by_schedule(&self, schedule_id: &str) -> Result<Facility, String> {
        self.facility_repo
            .get_facilities_by_schedule(schedule_id)
            .await
    }

    async fn get_by_user(&self, user_id: &str) -> Result<Vec<Facility>, String> {
        self.facility_repo.get_facilities_by_user(user_id).await
    }
}
