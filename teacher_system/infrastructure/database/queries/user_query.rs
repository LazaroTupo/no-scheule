use crate::domain::{models::user::User, repositories::user_repository::UserRepository};
use crate::infrastructure::database::entities::{
    course_schedules, courses, enrollments, facilities, sea_orm_active_enums, users,
};
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter,
    QuerySelect, RelationTrait, Set,
};
use shared::config::connect_to_supabase;

#[derive(Clone)]
pub struct SupabaseUserRepository {
    db: DatabaseConnection,
}
impl SupabaseUserRepository {
    pub async fn new() -> Result<Self, String> {
        let db = connect_to_supabase().await.map_err(|e| e.to_string())?;
        Ok(Self { db })
    }
}

#[async_trait]
impl UserRepository for SupabaseUserRepository {
    async fn get_user_by_id(&self, user_id: &str) -> Result<Option<User>, String> {
        users::Entity::find()
            .filter(users::Column::Id.eq(user_id))
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .map(|u| {
                Ok(User {
                    id: u.id,
                    code: u.code,
                    email: u.email,
                    phone: u.phone,
                    faculty: u.faculty,
                    program: u.program.unwrap_or_default(),
                    specialty: u.specialty,
                    role: u.role.to_string(),
                    student_status: u
                        .student_status
                        .map(|s| sea_orm_active_enums::to_domain_student_status(&s)),
                    admission_date: u.admission_date.map(|d| d.to_string()),
                    contract_type: u
                        .contract_type
                        .map(|c| sea_orm_active_enums::to_domain_contract(&c)),
                    max_hours_per_week: u.max_hours_per_week,
                    hire_date: u.hire_date.map(|d| d.to_string()),
                    full_name: u.full_name.unwrap_or_default(),
                })
            })
            .transpose()
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, String> {
        users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .map(|u| {
                Ok(User {
                    id: u.id,
                    code: u.code,
                    email: u.email,
                    phone: u.phone,
                    faculty: u.faculty,
                    program: u.program.unwrap_or_default(),
                    specialty: u.specialty,
                    role: u.role.to_string(),
                    student_status: u
                        .student_status
                        .map(|s| sea_orm_active_enums::to_domain_student_status(&s)),
                    admission_date: u.admission_date.map(|d| d.to_string()),
                    contract_type: u
                        .contract_type
                        .map(|c| sea_orm_active_enums::to_domain_contract(&c)),
                    max_hours_per_week: u.max_hours_per_week,
                    hire_date: u.hire_date.map(|d| d.to_string()),
                    full_name: u.full_name.unwrap_or_default(),
                })
            })
            .transpose()
    }

    async fn get_all_users(&self) -> Result<Vec<User>, String> {
        users::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|u| {
                Ok(User {
                    id: u.id,
                    code: u.code,
                    email: u.email,
                    phone: u.phone,
                    faculty: u.faculty,
                    program: u.program.unwrap_or_default(),
                    specialty: u.specialty,
                    role: u.role.to_string(),
                    student_status: u
                        .student_status
                        .map(|s| sea_orm_active_enums::to_domain_student_status(&s)),
                    admission_date: u.admission_date.map(|d| d.to_string()),
                    contract_type: u
                        .contract_type
                        .map(|c| sea_orm_active_enums::to_domain_contract(&c)),
                    max_hours_per_week: u.max_hours_per_week,
                    hire_date: u.hire_date.map(|d| d.to_string()),
                    full_name: u.full_name.unwrap_or_default(),
                })
            })
            .collect()
    }

    async fn create_user(&self, user: &User) -> Result<(), String> {
        let user_active_model = users::ActiveModel {
            id: Set(user.id.clone()),
            code: Set(user.code.clone()),
            email: Set(user.email.clone()),
            phone: Set(user.phone.clone()),
            faculty: Set(user.faculty.clone()),
            program: Set(Some(user.program.clone())),
            specialty: Set(user.specialty.clone()),
            role: Set(sea_orm_active_enums::UserRole::from_string(&user.role)
                .ok_or_else(|| "Invalid user role".to_string())?),
            student_status: Set(user
                .student_status
                .as_ref()
                .map(sea_orm_active_enums::to_db_student_status)),
            admission_date: user
                .admission_date
                .as_ref()
                .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
                .map(|d| Set(Some(d)))
                .unwrap_or_default(),
            contract_type: Set(user
                .contract_type
                .as_ref()
                .map(sea_orm_active_enums::to_db_contract)),
            max_hours_per_week: Set(user.max_hours_per_week),
            hire_date: user
                .hire_date
                .as_ref()
                .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
                .map(|d| Set(Some(d)))
                .unwrap_or_default(),
            full_name: Set(Some(user.full_name.clone())),
            password: Set("password".to_string()),
        };

        user_active_model
            .insert(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn update_user(&self, user: &User) -> Result<(), String> {
        let mut user_active_model: users::ActiveModel = users::Entity::find()
            .filter(users::Column::Id.eq(&user.id))
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "User not found".to_string())?
            .into();

        user_active_model.code = Set(user.code.clone());
        user_active_model.email = Set(user.email.clone());
        user_active_model.phone = Set(user.phone.clone());
        user_active_model.faculty = Set(user.faculty.clone());
        user_active_model.program = Set(Some(user.program.clone()));
        user_active_model.specialty = Set(user.specialty.clone());
        user_active_model.role = Set(sea_orm_active_enums::UserRole::from_string(&user.role)
            .ok_or_else(|| "Invalid user role".to_string())?);
        user_active_model.student_status = Set(user
            .student_status
            .as_ref()
            .map(sea_orm_active_enums::to_db_student_status));
        user_active_model.admission_date = user
            .admission_date
            .as_ref()
            .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
            .map(|d| Set(Some(d)))
            .unwrap_or_default();
        user_active_model.contract_type = Set(user
            .contract_type
            .as_ref()
            .map(sea_orm_active_enums::to_db_contract));
        user_active_model.max_hours_per_week = Set(user.max_hours_per_week);
        user_active_model.hire_date = user
            .hire_date
            .as_ref()
            .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
            .map(|d| Set(Some(d)))
            .unwrap_or_default();
        user_active_model.full_name = Set(Some(user.full_name.clone()));
        user_active_model.password = Set("password".to_string());

        user_active_model
            .update(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn delete_user(&self, user_id: &str) -> Result<(), String> {
        users::Entity::delete_by_id(user_id)
            .exec(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_users_by_course(&self, course_id: &str) -> Result<Vec<User>, String> {
        users::Entity::find()
            .join(JoinType::InnerJoin, users::Relation::Enrollments.def())
            .filter(enrollments::Column::CourseId.eq(course_id))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|u| {
                Ok(User {
                    id: u.id,
                    code: u.code,
                    email: u.email,
                    phone: u.phone,
                    faculty: u.faculty,
                    program: u.program.unwrap_or_default(),
                    specialty: u.specialty,
                    role: u.role.to_string(),
                    student_status: u
                        .student_status
                        .map(|s| sea_orm_active_enums::to_domain_student_status(&s)),
                    admission_date: u.admission_date.map(|d| d.to_string()),
                    contract_type: u
                        .contract_type
                        .map(|c| sea_orm_active_enums::to_domain_contract(&c)),
                    max_hours_per_week: u.max_hours_per_week,
                    hire_date: u.hire_date.map(|d| d.to_string()),
                    full_name: u.full_name.unwrap_or_default(),
                })
            })
            .collect()
    }

    async fn get_users_by_course_name(&self, name_course: &str) -> Result<Vec<User>, String> {
        users::Entity::find()
            .join(JoinType::InnerJoin, users::Relation::Enrollments.def())
            .join(JoinType::InnerJoin, enrollments::Relation::Courses.def())
            .filter(courses::Column::Name.eq(name_course))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|u| {
                Ok(User {
                    id: u.id,
                    code: u.code,
                    email: u.email,
                    phone: u.phone,
                    faculty: u.faculty,
                    program: u.program.unwrap_or_default(),
                    specialty: u.specialty,
                    role: u.role.to_string(),
                    student_status: u
                        .student_status
                        .map(|s| sea_orm_active_enums::to_domain_student_status(&s)),
                    admission_date: u.admission_date.map(|d| d.to_string()),
                    contract_type: u
                        .contract_type
                        .map(|c| sea_orm_active_enums::to_domain_contract(&c)),
                    max_hours_per_week: u.max_hours_per_week,
                    hire_date: u.hire_date.map(|d| d.to_string()),
                    full_name: u.full_name.unwrap_or_default(),
                })
            })
            .collect()
    }

    async fn get_users_by_facility(&self, facility_id: &str) -> Result<Vec<User>, String> {
        // Para estudiantes: a través de cursos en esa instalación
        let students = users::Entity::find()
            .join(JoinType::InnerJoin, users::Relation::Enrollments.def())
            .join(JoinType::InnerJoin, enrollments::Relation::Courses.def())
            .join(
                JoinType::InnerJoin,
                courses::Relation::CourseSchedules.def(),
            )
            .filter(course_schedules::Column::FacilityId.eq(facility_id))
            .filter(users::Column::Role.eq(sea_orm_active_enums::UserRole::Student))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        // Para profesores: a través de cursos que dictan en esa instalación
        let teachers = users::Entity::find()
            .join(JoinType::InnerJoin, users::Relation::Courses.def())
            .join(
                JoinType::InnerJoin,
                courses::Relation::CourseSchedules.def(),
            )
            .filter(course_schedules::Column::FacilityId.eq(facility_id))
            .filter(users::Column::Role.eq(sea_orm_active_enums::UserRole::Teacher))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        let mut all_users = students;
        all_users.extend(teachers);

        all_users
            .into_iter()
            .map(|u| {
                Ok(User {
                    id: u.id,
                    code: u.code,
                    email: u.email,
                    phone: u.phone,
                    faculty: u.faculty,
                    program: u.program.unwrap_or_default(),
                    specialty: u.specialty,
                    role: u.role.to_string(),
                    student_status: u
                        .student_status
                        .map(|s| sea_orm_active_enums::to_domain_student_status(&s)),
                    admission_date: u.admission_date.map(|d| d.to_string()),
                    contract_type: u
                        .contract_type
                        .map(|c| sea_orm_active_enums::to_domain_contract(&c)),
                    max_hours_per_week: u.max_hours_per_week,
                    hire_date: u.hire_date.map(|d| d.to_string()),
                    full_name: u.full_name.unwrap_or_default(),
                })
            })
            .collect()
    }

    async fn get_users_by_facility_name(&self, name_facility: &str) -> Result<Vec<User>, String> {
        // Para estudiantes: a través de cursos en esa instalación
        let students = users::Entity::find()
            .join(JoinType::InnerJoin, users::Relation::Enrollments.def())
            .join(JoinType::InnerJoin, enrollments::Relation::Courses.def())
            .join(
                JoinType::InnerJoin,
                courses::Relation::CourseSchedules.def(),
            )
            .join(
                JoinType::InnerJoin,
                course_schedules::Relation::Facilities.def(),
            )
            .filter(facilities::Column::Name.eq(name_facility))
            .filter(users::Column::Role.eq(sea_orm_active_enums::UserRole::Student))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        // Para profesores: a través de cursos que dictan en esa instalación
        let teachers = users::Entity::find()
            .join(JoinType::InnerJoin, users::Relation::Courses.def())
            .join(
                JoinType::InnerJoin,
                courses::Relation::CourseSchedules.def(),
            )
            .join(
                JoinType::InnerJoin,
                course_schedules::Relation::Facilities.def(),
            )
            .filter(facilities::Column::Name.eq(name_facility))
            .filter(users::Column::Role.eq(sea_orm_active_enums::UserRole::Teacher))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        let mut all_users = students;
        all_users.extend(teachers);

        all_users
            .into_iter()
            .map(|u| {
                Ok(User {
                    id: u.id,
                    code: u.code,
                    email: u.email,
                    phone: u.phone,
                    faculty: u.faculty,
                    program: u.program.unwrap_or_default(),
                    specialty: u.specialty,
                    role: u.role.to_string(),
                    student_status: u
                        .student_status
                        .map(|s| sea_orm_active_enums::to_domain_student_status(&s)),
                    admission_date: u.admission_date.map(|d| d.to_string()),
                    contract_type: u
                        .contract_type
                        .map(|c| sea_orm_active_enums::to_domain_contract(&c)),
                    max_hours_per_week: u.max_hours_per_week,
                    hire_date: u.hire_date.map(|d| d.to_string()),
                    full_name: u.full_name.unwrap_or_default(),
                })
            })
            .collect()
    }

    async fn get_users_by_schedule(&self, schedule_id: &str) -> Result<Vec<User>, String> {
        // Obtener el curso asociado al horario
        let course_id = course_schedules::Entity::find_by_id(schedule_id)
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Schedule not found".to_string())?
            .course_id;

        // Obtener usuarios matriculados en ese curso
        self.get_users_by_course(&course_id).await
    }

    async fn get_users_by_name(&self, name: &str) -> Result<Vec<User>, String> {
        users::Entity::find()
            .filter(users::Column::FullName.like(format!("%{}%", name)))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|u| {
                Ok(User {
                    id: u.id,
                    code: u.code,
                    email: u.email,
                    phone: u.phone,
                    faculty: u.faculty,
                    program: u.program.unwrap_or_default(),
                    specialty: u.specialty,
                    role: u.role.to_string(),
                    student_status: u
                        .student_status
                        .map(|s| sea_orm_active_enums::to_domain_student_status(&s)),
                    admission_date: u.admission_date.map(|d| d.to_string()),
                    contract_type: u
                        .contract_type
                        .map(|c| sea_orm_active_enums::to_domain_contract(&c)),
                    max_hours_per_week: u.max_hours_per_week,
                    hire_date: u.hire_date.map(|d| d.to_string()),
                    full_name: u.full_name.unwrap_or_default(),
                })
            })
            .collect()
    }
}
