use teacher_system::application::use_cases::{
    course_management::CourseManagementUseCase, schedule_management::ScheduleManagementUseCase,
    teacher_queries::TeacherQueriesUseCase,
};
use teacher_system::domain::services::{DefaultSchedulingService, DefaultValidationService};
use teacher_system::infrastructure::supabase::{
    SupabaseCourseRepository, SupabaseScheduleRepository, SupabaseUserRepository,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configuración inicial
    let supabase_url = std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let supabase_key = std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set");

    // Inicializar cliente Supabase
    let client = supabase_rs::SupabaseClient::new(supabase_url, supabase_key);

    // Inicializar repositorios
    let course_repo = SupabaseCourseRepository::new(client.clone());
    let schedule_repo = SupabaseScheduleRepository::new(client.clone());
    let user_repo = SupabaseUserRepository::new(client.clone());

    // Inicializar servicios
    let validation_service = DefaultValidationService::new(
        Box::new(course_repo.clone()),
        Box::new(schedule_repo.clone()),
        Box::new(user_repo.clone()),
    );

    let scheduling_service = DefaultSchedulingService::new(Box::new(course_repo.clone()));

    // Inicializar casos de uso
    let course_uc = CourseManagementUseCase::new(&course_repo, &validation_service);
    let schedule_uc = ScheduleManagementUseCase::new(&schedule_repo, &validation_service);
    let teacher_uc = TeacherQueriesUseCase::new(&user_repo, &course_repo);

    // Aquí iría la lógica de la aplicación (API, CLI, etc.)

    Ok(())
}
