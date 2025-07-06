use std::sync::Arc;

use crate::domain::services::validation_service::DefaultValidationService;
use crate::infrastructure::database::queries::course_query::SupabaseCourseRepository;
use crate::infrastructure::database::queries::facility_query::SupabaseFacilityRepository;
use crate::infrastructure::database::queries::schedule_query::SupabaseScheduleRepository;
use crate::infrastructure::database::queries::user_query::SupabaseUserRepository;
use crate::{
    application::use_cases::{
        course_management::CourseManagementUseCaseImpl, facility_management::FacilityManagementUseCaseImpl, schedule_management::ScheduleManagementUseCaseImpl,
        user_management::UserManagementUseCaseImpl,
    },
    domain::services::scheduling_service::DefaultSchedulingService,
};

#[derive(Clone)]
pub struct AppState {
    pub course_use_case: Arc<CourseManagementUseCaseImpl>,
    pub facility_use_case: Arc<FacilityManagementUseCaseImpl>,
    pub schedule_use_case: Arc<ScheduleManagementUseCaseImpl>,
    pub user_use_case: Arc<UserManagementUseCaseImpl>,
}

pub async fn bootstrap_teacher() -> Result<AppState, String> {
    // Course_case
    let course_repo = SupabaseCourseRepository::new().await?;
    let schedule_repo = SupabaseScheduleRepository::new().await?;
    let validation_service = DefaultValidationService::new(Arc::new(schedule_repo.clone()));

    // Facility_case
    let facility_repo = SupabaseFacilityRepository::new().await?;

    // Schedule_case
    let schedule_repo = SupabaseScheduleRepository::new().await?;
    let scheduling_service = DefaultSchedulingService::new(Arc::new(course_repo.clone()), Arc::new(schedule_repo.clone()));

    // User_case
    let user_repo = SupabaseUserRepository::new().await?;

    //// Instanciar los use_Case
    let course_use_case = Arc::new(CourseManagementUseCaseImpl::new(Box::new(course_repo.clone()), validation_service.clone()));
    let facility_use_case = Arc::new(FacilityManagementUseCaseImpl::new(Box::new(facility_repo.clone())));
    let schedule_use_case = Arc::new(ScheduleManagementUseCaseImpl::new(
        Box::new(schedule_repo.clone()),
        validation_service.clone(),
        scheduling_service.clone(),
    ));
    let user_use_case = Arc::new(UserManagementUseCaseImpl::new(Box::new(user_repo.clone())));

    // Crear estado de aplicación
    Ok(AppState {
        course_use_case,
        facility_use_case,
        schedule_use_case,
        user_use_case,
    })
}
