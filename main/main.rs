use dotenv::dotenv;
use std::sync::Arc;
use teacher_system::application::use_cases::{
    course_management::CourseManagementUseCase, schedule_management::ScheduleManagementUseCase,
    teacher_queries::TeacherQueriesUseCase,
};
use teacher_system::domain::services::{
    scheduling_service::DefaultSchedulingService, validation_service::DefaultValidationService,
};
use teacher_system::infrastructure::supabase::{
    course_repo_impl::SupabaseCourseRepository, schedule_repo_impl::SupabaseScheduleRepository,
    user_repo_impl::SupabaseUserRepository,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    // Configuración inicial
    let supabase_url = std::env::var("SUPABASE_URL").expect("SUPABASE_URL debe ser configurado");
    let supabase_key = std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY debe ser configurado");

    // Inicializar cliente Supabase
    let client = match supabase_rs::SupabaseClient::new(supabase_url, supabase_key) {
        Ok(client) => Arc::new(client),
        Err(err) => {
            eprintln!("Error al inicializar SupabaseClient: {:?}", err);
            std::process::exit(1);
        }
    };

    // Inicializar repositorios
    let course_repo = SupabaseCourseRepository::new(client.clone());
    let schedule_repo = SupabaseScheduleRepository::new(client.clone());
    let user_repo = SupabaseUserRepository::new(client.clone());

    // Inicializar servicios
    let validation_service = DefaultValidationService::new(
        Arc::new(course_repo.clone()),
        Arc::new(schedule_repo.clone()),
        Arc::new(user_repo.clone()),
    );

    let scheduling_service = DefaultSchedulingService::new(Arc::new(course_repo.clone()));

    // Inicializar casos de uso
    let course_uc = CourseManagementUseCase::new(&course_repo, &validation_service);
    let schedule_uc = ScheduleManagementUseCase::new(&schedule_repo, &validation_service);
    let teacher_uc = TeacherQueriesUseCase::new(&user_repo, &course_repo);

    // Aquí iría la lógica de la aplicación (API, CLI, etc.)
    print!("Aplicación de gestión de profesores iniciada correctamente.\n");
    Ok(())
}
