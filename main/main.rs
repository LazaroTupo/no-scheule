use student_system;
use teacher_system;

use teacher_system::infrastructure::supabase::{SupabaseCourseRepository, SupabaseScheduleRepository};
use teacher_system::domain::services::scheduling::DefaultSchedulingService;
use teacher_system::application::use_cases::course_management::CourseManagementUseCase;

#[tokio::main]
async fn main() {
    // Configurar cliente Supabase
    let supabase_client = SupabaseClient::new("URL", "KEY");
    
    // Inicializar repositorios
    let course_repo = SupabaseCourseRepository::new(supabase_client.clone());
    let schedule_repo = SupabaseScheduleRepository::new(supabase_client.clone());
    
    // Inicializar servicios
    let scheduling_service = DefaultSchedulingService::new(Box::new(course_repo));
    
    // Inicializar casos de uso
    let course_use_case = CourseManagementUseCase::new(
        &course_repo,
        &scheduling_service,
    );
    
    // Ejemplo: Registrar un nuevo curso
    let new_course = Course {
        id: "course1".to_string(),
        code: "CS101".to_string(),
        name: "Computer Science 101".to_string(),
        teacher_id: "teacher1".to_string(),
        // otros campos
    };
    
    let schedule = Schedule {
        day: Weekday::Mon,
        start_time: "10:00".parse().unwrap(),
        end_time: "12:00".parse().unwrap(),
        // otros campos
    };
    
    match course_use_case.register_course(new_course, schedule).await {
        Ok(_) => println!("Curso registrado exitosamente"),
        Err(e) => eprintln!("Error: {}", e),
    }
}