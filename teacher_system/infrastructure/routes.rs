use super::controllers::{courses_controller, schedules_controller, teachers_controller};
use crate::domain::repositories::{
    course_repository::CourseRepository, schedule_repository::ScheduleRepository,
    user_repository::UserRepository,
};
use axum::{
    Router,
    routing::{get, post, put},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    course_repo: Arc<dyn CourseRepository>,
    schedule_repo: Arc<dyn ScheduleRepository>,
    user_repo: Arc<dyn UserRepository>,
}

pub fn configure_routes(
    course_repo: Arc<dyn CourseRepository>,
    schedule_repo: Arc<dyn ScheduleRepository>,
    user_repo: Arc<dyn UserRepository>,
) -> Router {
    Router::new()
        .route("/api/courses", post(courses_controller::create_course))
        .route("/api/courses/:id", get(courses_controller::get_course))
        .route(
            "/api/teachers/:id",
            get(teachers_controller::get_teacher_info),
        )
        .route(
            "/api/teachers/:id/schedule",
            get(teachers_controller::get_teacher_schedule),
        )
        .route(
            "/api/schedules",
            post(schedules_controller::create_schedule),
        )
        .with_state(AppState {
            course_repo,
            schedule_repo,
            user_repo,
        })
}
