use crate::application::dto::{TeacherDTO, TeacherScheduleDTO};
use crate::domain::repositories::{CourseRepository, UserRepository};

pub struct TeacherQueriesUseCase<'a> {
    user_repo: &'a dyn UserRepository,
    course_repo: &'a dyn CourseRepository,
}

impl<'a> TeacherQueriesUseCase<'a> {
    pub fn new(user_repo: &'a dyn UserRepository, course_repo: &'a dyn CourseRepository) -> Self {
        Self {
            user_repo,
            course_repo,
        }
    }

    pub async fn get_teacher_info(&self, teacher_id: &str) -> Result<TeacherDTO, String> {
        let teacher = self.user_repo.get_teacher_details(teacher_id).await?;

        let current_hours = self.calculate_teacher_hours(teacher_id).await?;

        Ok(TeacherDTO {
            id: teacher.id,
            code: teacher.code,
            name: format!(
                "{} {}",
                teacher.first_name.unwrap_or_default(),
                teacher.last_name.unwrap_or_default()
            ),
            email: teacher.email.unwrap_or_default(),
            faculty: teacher.faculty,
            max_hours_per_week: teacher.max_hours_per_week.unwrap_or(0),
            current_hours,
        })
    }

    async fn calculate_teacher_hours(&self, teacher_id: &str) -> Result<i32, String> {
        let courses = self.course_repo.get_teacher_courses(teacher_id).await?;
        let total_hours = courses.iter().map(|c| c.hours_per_week).sum();
        Ok(total_hours)
    }
}
