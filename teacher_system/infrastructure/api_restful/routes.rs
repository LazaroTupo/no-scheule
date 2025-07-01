// src/infrastructure/api/routes.rs

use crate::infrastructure::api_restful::controllers::{
    course_controller, facility_controller, schedule_controller,
};
use actix_web::{Scope, web};

/// Retorna un `Scope` con todas las rutas agrupadas por entidad
pub fn app_routes() -> Scope {
    web::scope("/api")
        .service(course_routes())
        .service(schedule_routes())
        .service(facility_routes())
}

/// Rutas relacionadas a `Course`
fn course_routes() -> Scope {
    web::scope("/courses")
        .route("", web::get().to(course_controller::get_all_courses))
        .route("", web::post().to(course_controller::create_course))
        .route("/{id}", web::get().to(course_controller::get_course_by_id))
        .route("/{id}", web::put().to(course_controller::update_course))
        .route("/{id}", web::delete().to(course_controller::delete_course))
}

/// Rutas relacionadas a `Schedule`
fn schedule_routes() -> Scope {
    web::scope("/schedules")
        .route("", web::get().to(schedule_controller::get_all_schedules))
        .route("", web::post().to(schedule_controller::create_schedule))
        .route(
            "/{id}",
            web::get().to(schedule_controller::get_schedule_by_id),
        )
        .route("/{id}", web::put().to(schedule_controller::update_schedule))
        .route(
            "/{id}",
            web::delete().to(schedule_controller::delete_schedule),
        )
        // Operaciones específicas
        .route(
            "/suggest/{teacher_id}",
            web::post().to(schedule_controller::suggest_available_times),
        )
}

/// Rutas relacionadas a `Facility`
fn facility_routes() -> Scope {
    web::scope("/facilities")
        .route("", web::get().to(facility_controller::get_all_facilities))
        .route("", web::post().to(facility_controller::create_facility))
        .route(
            "/{id}",
            web::get().to(facility_controller::get_facility_by_id),
        )
        .route("/{id}", web::put().to(facility_controller::update_facility))
        .route(
            "/{id}",
            web::delete().to(facility_controller::delete_facility),
        )
}
