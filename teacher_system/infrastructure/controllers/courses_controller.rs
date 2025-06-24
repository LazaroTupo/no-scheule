use crate::{
    application::dto::course_dto::{CourseDTO, CourseResponseDTO},
    application::use_cases::course_management::CourseManagementUseCase,
    domain::models::course::Course,
    domain::models::schedule::Schedule,
    domain::repositories::course_repository::CourseRepository,
    infrastructure::error::ApiError,
    infrastructure::routes::AppState,
};
use axum::{
    Json,
    extract::{Path, State},
};
use std::sync::Arc;

pub async fn create_course(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CourseDTO>,
) -> Result<Json<CourseResponseDTO>, ApiError> {
    let course_uc = CourseManagementUseCase::new(&state.course_repo, &state.validation_service);

    let course = Course::from(payload);
    let schedule = Schedule::default(); // Deberías recibir esto en el DTO

    course_uc
        .register_course(course, schedule)
        .await
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    // Convertir a DTO de respuesta
    Ok(Json(CourseResponseDTO::from(course)))
}

pub async fn get_course(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<CourseResponseDTO>, ApiError> {
    let course = state
        .course_repo
        .get_course_by_id(&id)
        .await
        .map_err(|_| ApiError::InternalServerError("Failed to fetch course".into()))?;

    match course {
        Some(c) => Ok(Json(CourseResponseDTO::from(c))),
        None => Err(ApiError::NotFound("Course not found".into())),
    }
}
