use crate::domain::{
    models::{Course, Schedule},
    repositories::CourseRepository,
    services::ValidationService,
};

pub struct CourseManagementUseCase<'a> {
    course_repo: &'a dyn CourseRepository,
    validation_service: &'a dyn ValidationService,
}

impl<'a> CourseManagementUseCase<'a> {
    pub fn new(
        course_repo: &'a dyn CourseRepository,
        validation_service: &'a dyn ValidationService,
    ) -> Self {
        Self {
            course_repo,
            validation_service,
        }
    }

    pub async fn register_course(&self, course: Course, schedule: Schedule) -> Result<(), String> {
        // Validar disponibilidad
        let has_conflict = self
            .validation_service
            .check_teacher_availability(&course.teacher_id, &schedule)
            .await?;

        if has_conflict {
            return Err("El profesor ya tiene un curso en ese horario".into());
        }

        // Crear curso
        self.course_repo.create_course(&course).await
    }
}
