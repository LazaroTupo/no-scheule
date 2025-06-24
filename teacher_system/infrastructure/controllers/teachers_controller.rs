use crate::{
    api::error::ApiError,
    application::dto::course_dto::CourseResponseDTO,
    application::dto::teacher_dto::{TeacherDTO, TeacherScheduleDTO},
    application::use_cases::schedule_management::ScheduleManagementUseCase,
    application::use_cases::teacher_queries::TeacherQueriesUseCase,
    domain::repositories::course_repository::CourseRepository,
    domain::repositories::user_repository::UserRepository,
    infrastructure::routes::AppState,
};

use axum::{
    Json,
    extract::{Path, State},
};
use std::sync::Arc;

pub async fn get_teacher_info(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<TeacherDTO>, ApiError> {
    let teacher_uc = TeacherQueriesUseCase::new(&state.user_repo, &state.course_repo);

    let teacher = teacher_uc
        .get_teacher_info(&id)
        .await
        .map_err(|e| ApiError::NotFound(e.to_string()))?;

    Ok(Json(teacher))
}

pub async fn get_teacher_schedule(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<TeacherScheduleDTO>, ApiError> {
    let teacher_uc = TeacherQueriesUseCase::new(&state.user_repo, &state.course_repo);
    let schedule_uc =
        ScheduleManagementUseCase::new(&state.schedule_repo, &state.validation_service);

    let courses = teacher_uc
        .get_teacher_courses(&id)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    let extracurricular = schedule_uc
        .get_teacher_extracurricular(&id)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(Json(TeacherScheduleDTO {
        courses: courses.into_iter().map(CourseResponseDTO::from).collect(),
        extracurricular: extracurricular
            .into_iter()
            .map(ScheduleResponseDTO::from)
            .collect(),
    }))
}
