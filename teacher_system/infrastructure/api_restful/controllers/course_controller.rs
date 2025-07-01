use crate::domain::models::course::Course;
use crate::{
    application::use_cases::course_management::CourseManagementUseCase as CourseUseCase,
    domain::models::schedule::Schedule,
};
use actix_web::{Error, HttpResponse, web};

pub async fn register_extracourse(
    use_case: web::Data<dyn CourseUseCase>,
    schedule: web::Json<Schedule>,
    course: web::Json<Course>,
) -> Result<HttpResponse, Error> {
    let course_data = course.into_inner();
    let schedule = schedule.into_inner();
    use_case
        .register_extracourse(course_data, schedule)
        .await
        .map_err(|e| {
            eprintln!("Error registering extracourse: {}", e);
            actix_web::error::ErrorInternalServerError("Internal server error")
        })?;
    Ok(HttpResponse::Created().finish())
}

pub async fn get_all_courses(
    use_case: web::Data<dyn CourseUseCase>,
) -> Result<HttpResponse, Error> {
    let courses = use_case.get_all().await.map_err(|e| {
        eprintln!("Error fetching courses: {}", e);
        actix_web::error::ErrorInternalServerError("Internal server error")
    })?;
    Ok(HttpResponse::Ok().json(courses))
}

pub async fn create_course(
    use_case: web::Data<dyn CourseUseCase>,
    new_course: web::Json<Course>,
) -> Result<HttpResponse, Error> {
    let course = use_case
        .create(new_course.into_inner())
        .await
        .map_err(|e| {
            eprintln!("Error creating course: {}", e);
            actix_web::error::ErrorInternalServerError("Internal server error")
        })?;
    Ok(HttpResponse::Created().json(course))
}

pub async fn get_course_by_id(
    use_case: web::Data<dyn CourseUseCase>,
    id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    match use_case.get_by_id(&id).await {
        Ok(course) => Ok(HttpResponse::Ok().json(course)),
        Err(e) if e.to_lowercase().contains("not found") => {
            Ok(HttpResponse::NotFound().body("Course not found"))
        }
        Err(e) => {
            eprintln!("Error fetching course: {}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Internal server error",
            ))
        }
    }
}

pub async fn update_course(
    use_case: web::Data<dyn CourseUseCase>,
    id: web::Path<String>,
    updated_course: web::Json<Course>,
) -> Result<HttpResponse, Error> {
    let mut course_data = updated_course.into_inner();
    course_data.id = id.clone(); // Aseguramos que el ID venga del path

    match use_case.update(&course_data).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) if e.to_lowercase().contains("not found") => {
            Ok(HttpResponse::NotFound().body("Course not found"))
        }
        Err(e) => {
            eprintln!("Error updating course: {}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Internal server error",
            ))
        }
    }
}

pub async fn delete_course(
    use_case: web::Data<dyn CourseUseCase>,
    id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    match use_case.delete(&id).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) if e.to_lowercase().contains("not found") => {
            Ok(HttpResponse::NotFound().body("Course not found"))
        }
        Err(e) => {
            eprintln!("Error deleting course: {}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Internal server error",
            ))
        }
    }
}
