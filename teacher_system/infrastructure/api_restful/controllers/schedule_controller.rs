use super::super::config::boostrap::AppState;
use crate::application::use_cases::schedule_management::ScheduleManagementUseCase;
use crate::domain::{models::enums::Weekday, models::schedule::Schedule};
use actix_web::{Error, HttpResponse, web};

// Operaciones CRUD básicas
pub async fn get_all_schedules(use_case: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let schedules = use_case.schedule_use_case.get_all().await.map_err(|e| {
        eprintln!("Error fetching schedules: {}", e);
        actix_web::error::ErrorInternalServerError("Internal server error")
    })?;
    Ok(HttpResponse::Ok().json(schedules))
}

pub async fn create_schedule(use_case: web::Data<AppState>, new_schedule: web::Json<Schedule>) -> Result<HttpResponse, Error> {
    let schedule = use_case.schedule_use_case.create(new_schedule.into_inner()).await.map_err(|e| {
        eprintln!("Error creating schedule: {}", e);
        actix_web::error::ErrorInternalServerError("Internal server error")
    })?;
    Ok(HttpResponse::Created().json(schedule))
}

pub async fn get_schedule_by_id(use_case: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    match use_case.schedule_use_case.get_by_id(&id).await {
        Ok(schedule) => Ok(HttpResponse::Ok().json(schedule)),
        Err(e) if e.to_lowercase().contains("not found") => Ok(HttpResponse::NotFound().body("Schedule not found")),
        Err(e) => {
            eprintln!("Error fetching schedule: {}", e);
            Err(actix_web::error::ErrorInternalServerError("Internal server error"))
        }
    }
}

pub async fn update_schedule(use_case: web::Data<AppState>, id: web::Path<String>, updated_schedule: web::Json<Schedule>) -> Result<HttpResponse, Error> {
    let mut schedule = updated_schedule.into_inner();
    schedule.id = id.clone();

    match use_case.schedule_use_case.update(&schedule).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) if e.to_lowercase().contains("not found") => Ok(HttpResponse::NotFound().body("Schedule not found")),
        Err(e) if e.to_lowercase().contains("not available") => Ok(HttpResponse::Conflict().body(e)),
        Err(e) => {
            eprintln!("Error updating schedule: {}", e);
            Err(actix_web::error::ErrorInternalServerError("Internal server error"))
        }
    }
}

pub async fn delete_schedule(use_case: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    match use_case.schedule_use_case.delete(&id).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) if e.to_lowercase().contains("not found") => Ok(HttpResponse::NotFound().body("Schedule not found")),
        Err(e) => {
            eprintln!("Error deleting schedule: {}", e);
            Err(actix_web::error::ErrorInternalServerError("Internal server error"))
        }
    }
}

// Operaciones específicas de gestión
pub async fn suggest_available_times(use_case: web::Data<AppState>, teacher_id: web::Path<String>, payload: web::Json<(i32, Vec<Weekday>)>) -> Result<HttpResponse, Error> {
    let (duration_minutes, preferred_days) = payload.into_inner();

    match use_case.schedule_use_case.suggest_available_times(&teacher_id, duration_minutes, preferred_days).await {
        Ok(suggestions) => Ok(HttpResponse::Ok().json(suggestions)),
        Err(e) => {
            eprintln!("Error suggesting times: {}", e);
            Err(actix_web::error::ErrorInternalServerError("Internal server error"))
        }
    }
}
