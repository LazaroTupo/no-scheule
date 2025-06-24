use crate::{
    api::error::ApiError,
    application::dto::schedule_dto::{ScheduleDTO, ScheduleResponseDTO},
    application::use_cases::schedule_management::ScheduleManagementUseCase,
    infrastructure::routes::AppState,
};
use axum::{Json, extract::State};
use std::sync::Arc;

pub async fn create_schedule(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ScheduleDTO>,
) -> Result<Json<ScheduleResponseDTO>, ApiError> {
    let schedule_uc =
        ScheduleManagementUseCase::new(&state.schedule_repo, &state.validation_service);

    let schedule = schedule_uc
        .set_course_schedule(&payload.course_id, payload)
        .await
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    Ok(Json(ScheduleResponseDTO::from(schedule)))
}
