// src/infrastructure/api_restful/controllers/facility_controller.rs

use super::super::config::boostrap::AppState;
use crate::application::use_cases::facility_management::FacilityManagementUseCase;
use crate::domain::models::facilitie::Facility;
use actix_web::{Error, HttpResponse, web};

pub async fn get_all_facilities(use_case: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let facilities = use_case.facility_use_case.get_all().await.map_err(|e| {
        eprintln!("Error fetching facilities: {}", e);
        actix_web::error::ErrorInternalServerError("Internal server error")
    })?;
    Ok(HttpResponse::Ok().json(facilities))
}

pub async fn create_facility(use_case: web::Data<AppState>, new_facility: web::Json<Facility>) -> Result<HttpResponse, Error> {
    let facility = use_case.facility_use_case.create(new_facility.into_inner()).await.map_err(|e| {
        eprintln!("Error creating facility: {}", e);
        actix_web::error::ErrorInternalServerError("Internal server error")
    })?;
    Ok(HttpResponse::Created().json(facility))
}

pub async fn get_facility_by_id(use_case: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    match use_case.facility_use_case.get_by_id(&id).await {
        Ok(facility) => Ok(HttpResponse::Ok().json(facility)),
        Err(e) if e.to_lowercase().contains("not found") => Ok(HttpResponse::NotFound().body("Facility not found")),
        Err(e) => {
            eprintln!("Error fetching facility: {}", e);
            Err(actix_web::error::ErrorInternalServerError("Internal server error"))
        }
    }
}

pub async fn update_facility(use_case: web::Data<AppState>, id: web::Path<String>, updated_facility: web::Json<Facility>) -> Result<HttpResponse, Error> {
    let mut facility_data = updated_facility.into_inner();
    facility_data.id = id.clone(); // asegurar que el ID del path prevalece

    match use_case.facility_use_case.update(&facility_data).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) if e.to_lowercase().contains("not found") => Ok(HttpResponse::NotFound().body("Facility not found")),
        Err(e) => {
            eprintln!("Error updating facility: {}", e);
            Err(actix_web::error::ErrorInternalServerError("Internal server error"))
        }
    }
}

pub async fn delete_facility(use_case: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    match use_case.facility_use_case.delete(&id).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) if e.to_lowercase().contains("not found") => Ok(HttpResponse::NotFound().body("Facility not found")),
        Err(e) => {
            eprintln!("Error deleting facility: {}", e);
            Err(actix_web::error::ErrorInternalServerError("Internal server error"))
        }
    }
}
