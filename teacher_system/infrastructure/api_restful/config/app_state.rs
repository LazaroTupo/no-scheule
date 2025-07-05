// src/infrastructure/config/app_state.rs

use crate::application::use_cases::{
    course_management::CourseManagementUseCaseImpl,
    facility_management::FacilityManagementUseCaseImpl,
    schedule_management::ScheduleManagementUseCaseImpl, user_management::UserManagementUseCaseImpl,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub course_use_case: Arc<CourseManagementUseCaseImpl>,
    pub facility_use_case: Arc<FacilityManagementUseCaseImpl>,
    pub schedule_use_case: Arc<ScheduleManagementUseCaseImpl>,
    pub user_use_case: Arc<UserManagementUseCaseImpl>,
}
