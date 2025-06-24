pub mod auth;
pub mod controllers;
pub mod error;
pub mod routes;
pub mod supabase;

use axum::Router;
use std::sync::Arc;

pub async fn create_api_router(
    course_repo: Arc<dyn crate::domain::repositories::course_repository::CourseRepository>,
    schedule_repo: Arc<dyn crate::domain::repositories::schedule_repository::ScheduleRepository>,
    user_repo: Arc<dyn crate::domain::repositories::user_repository::UserRepository>,
) -> Router {
    routes::configure_routes(course_repo, schedule_repo, user_repo)
}
