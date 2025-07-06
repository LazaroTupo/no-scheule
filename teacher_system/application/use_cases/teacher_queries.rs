use crate::application::dto::teacher_dto::TeacherDTO;
use crate::domain::repositories::{
    course_repository::CourseRepository, user_repository::UserRepository,
};

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
        println!("Called get_teacher_info with teacher_id={}", teacher_id);
        Err("Not implemented yet".to_string())
    }

    async fn calculate_teacher_hours(&self, teacher_id: &str) -> Result<i32, String> {
        let courses = self.course_repo.get_teacher_courses(teacher_id).await?;
        let total_hours = courses.iter().map(|c| c.hours_per_week).sum();
        Ok(total_hours)
    }
}
