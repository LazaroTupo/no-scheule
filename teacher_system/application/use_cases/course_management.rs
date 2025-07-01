use crate::domain::{
    models::{course::Course, schedule::Schedule},
    repositories::course_repository::CourseRepository,
    services::validation_service::ValidationService,
};
use async_trait::async_trait;
use std::sync::Arc;
#[async_trait]
pub trait CourseManagementUseCase {
    async fn register_extracourse(&self, course: Course, schedule: Schedule) -> Result<(), String>;
    async fn get_all(&self) -> Result<Vec<Course>, String>;
    async fn get_by_id(&self, id: &str) -> Result<Course, String>;
    async fn create(&self, course: Course) -> Result<(), String>;
    async fn update(&self, course: &Course) -> Result<(), String>;
    async fn delete(&self, id: &str) -> Result<(), String>;
    async fn get_by_user(&self, user_id: &str) -> Result<Vec<Course>, String>;
    async fn get_by_facility(&self, facility_id: &str) -> Result<Vec<Course>, String>;
    async fn get_by_facility_name(&self, name_facility: &str) -> Result<Vec<Course>, String>;
    async fn get_by_schedule(&self, schedule_id: &str) -> Result<Course, String>;
}

pub struct CourseManagementUseCaseImpl {
    course_repo: Box<dyn CourseRepository + Send + Sync>,
    validation_service: Arc<dyn ValidationService>,
}

impl CourseManagementUseCaseImpl {
    pub fn new(
        course_repo: Box<dyn CourseRepository + Send + Sync>,
        validation_service: Arc<dyn ValidationService>,
    ) -> Self {
        Self {
            course_repo,
            validation_service,
        }
    }
}

#[async_trait]
impl CourseManagementUseCase for CourseManagementUseCaseImpl {
    async fn register_extracourse(&self, course: Course, schedule: Schedule) -> Result<(), String> {
        let has_conflict = self
            .validation_service
            .check_teacher_availability(&course.teacher_id, &schedule)
            .await?;

        if has_conflict {
            return Err("El profesor ya tiene un curso en ese horario".into());
        }

        self.course_repo.create_course(&course).await
    }

    async fn get_all(&self) -> Result<Vec<Course>, String> {
        self.course_repo.get_all_courses().await
    }

    async fn get_by_id(&self, id: &str) -> Result<Course, String> {
        self.course_repo.get_course_by_id(id).await
    }

    async fn create(&self, course: Course) -> Result<(), String> {
        self.course_repo.create_course(&course).await
    }

    async fn update(&self, course: &Course) -> Result<(), String> {
        self.course_repo.update_course(course).await
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        self.course_repo.delete_course(id).await
    }

    async fn get_by_user(&self, user_id: &str) -> Result<Vec<Course>, String> {
        self.course_repo.get_courses_by_user(user_id).await
    }

    async fn get_by_facility(&self, facility_id: &str) -> Result<Vec<Course>, String> {
        self.course_repo.get_courses_by_facility(facility_id).await
    }

    async fn get_by_facility_name(&self, name_facility: &str) -> Result<Vec<Course>, String> {
        self.course_repo
            .get_courses_by_facility_name(name_facility)
            .await
    }

    async fn get_by_schedule(&self, schedule_id: &str) -> Result<Course, String> {
        self.course_repo.get_courses_by_schedule(schedule_id).await
    }
}
