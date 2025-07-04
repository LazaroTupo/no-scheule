use crate::domain::{models::course::Course, repositories::course_repository::CourseRepository};
use crate::infrastructure::database::entities::{
    course_schedules, courses, facilities, sea_orm_active_enums,
};
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, JoinType, ModelTrait,
    QueryFilter, QuerySelect, RelationTrait, Set,
};
use shared::config::connect_to_supabase;

#[derive(Clone)]
pub struct SupabaseCourseRepository {
    db: DatabaseConnection,
}

impl SupabaseCourseRepository {
    pub async fn new() -> Result<Self, String> {
        let db = connect_to_supabase().await.map_err(|e| e.to_string())?;
        Ok(Self { db })
    }

    // Función auxiliar para asignar aula automáticamente
    async fn assign_facility(&self, course: &Course) -> Result<String, String> {
        // Lógica simple para encontrar un aula disponible
        // En una implementación real, considerarías capacidad, horarios, etc.

        // Buscar aulas con capacidad suficiente
        let facility = facilities::Entity::find()
            .filter(facilities::Column::Capacity.gte(course.capacity))
            .filter(facilities::Column::FacilityType.eq("classroom"))
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        match facility {
            Some(f) => Ok(f.id),
            None => Err("No hay aulas disponibles que cumplan los requisitos".to_string()),
        }
    }
}

#[async_trait]
impl CourseRepository for SupabaseCourseRepository {
    async fn create_course(&self, course: &Course) -> Result<(), String> {
        let new_course = courses::ActiveModel {
            id: Set(course.id.clone()),
            code: Set(course.code.clone()),
            name: Set(course.name.clone()),
            section: Set(course.section),
            curriculum: Set(sea_orm_active_enums::to_db_curriculum(&course.curriculum)),
            capacity: Set(course.capacity),
            credits: Set(course.credits),
            hours_per_week: Set(course.hours_per_week),
            cycle: Set(course.cycle),
            teacher_id: Set(course.teacher_id.clone()),
            facility_id: Set(self.assign_facility(course).await?), // Campo temporal
            enrolled: Set(course.enrolled),
            max_capacity: Set(Some(course.capacity)),
            available_spots: Set(Some(course.available_spots())),
            semester: Set(course.semester.clone()),
            academic_year: Set(course.academic_year),
            active: Set(Some(course.active)),
            created_at: Set(Some(Utc::now().naive_utc())),
            updated_at: Set(Some(Utc::now().naive_utc())),
        };

        new_course
            .insert(&self.db)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn update_course(&self, course: &Course) -> Result<(), String> {
        let mut course_to_update: courses::ActiveModel = courses::Entity::find_by_id(&course.id)
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Curso no encontrado".to_string())?
            .into();

        course_to_update.code = Set(course.code.clone());
        course_to_update.name = Set(course.name.clone());
        course_to_update.section = Set(course.section);
        course_to_update.curriculum =
            Set(sea_orm_active_enums::to_db_curriculum(&course.curriculum));
        course_to_update.capacity = Set(course.capacity);
        course_to_update.credits = Set(course.credits);
        course_to_update.hours_per_week = Set(course.hours_per_week);
        course_to_update.cycle = Set(course.cycle);
        course_to_update.teacher_id = Set(course.teacher_id.clone());
        course_to_update.facility_id = Set(self.assign_facility(course).await?); // Campo temporal
        course_to_update.enrolled = Set(course.enrolled);
        course_to_update.max_capacity = Set(Some(course.capacity));
        course_to_update.available_spots = Set(Some(course.available_spots()));
        course_to_update.semester = Set(course.semester.clone());
        course_to_update.academic_year = Set(course.academic_year);
        course_to_update.active = Set(Some(course.active));
        course_to_update.updated_at = Set(Some(Utc::now().naive_utc()));

        course_to_update
            .update(&self.db)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_course_by_id(&self, id: &str) -> Result<Course, String> {
        let course_model = courses::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Curso no encontrado".to_string())?;

        Ok(Course {
            id: course_model.id,
            code: course_model.code,
            name: course_model.name,
            section: course_model.section,
            curriculum: sea_orm_active_enums::to_domain_curriculum(&course_model.curriculum),
            capacity: course_model.capacity,
            credits: course_model.credits,
            hours_per_week: course_model.hours_per_week,
            cycle: course_model.cycle,
            teacher_id: course_model.teacher_id,
            facility_id: "".to_string(), // Campo temporal
            enrolled: course_model.enrolled,
            semester: course_model.semester,
            academic_year: course_model.academic_year,
            active: course_model.active.unwrap_or(true),
        })
    }

    async fn get_all_courses(&self) -> Result<Vec<Course>, String> {
        let courses = courses::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(courses
            .into_iter()
            .map(|c| Course {
                id: c.id,
                code: c.code,
                name: c.name,
                section: c.section,
                curriculum: sea_orm_active_enums::to_domain_curriculum(&c.curriculum),
                capacity: c.capacity,
                credits: c.credits,
                hours_per_week: c.hours_per_week,
                cycle: c.cycle,
                teacher_id: c.teacher_id,
                facility_id: "".to_string(), // Campo temporal
                enrolled: c.enrolled,
                semester: c.semester,
                academic_year: c.academic_year,
                active: c.active.unwrap_or(true),
            })
            .collect())
    }

    async fn get_courses_by_user(&self, user_id: &str) -> Result<Vec<Course>, String> {
        // Para estudiantes: cursos en los que están matriculados
        // Para profesores: cursos que enseñan
        let courses = courses::Entity::find()
            .filter(courses::Column::TeacherId.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(courses
            .into_iter()
            .map(|c| Course {
                id: c.id,
                code: c.code,
                name: c.name,
                section: c.section,
                curriculum: sea_orm_active_enums::to_domain_curriculum(&c.curriculum),
                capacity: c.capacity,
                credits: c.credits,
                hours_per_week: c.hours_per_week,
                cycle: c.cycle,
                teacher_id: c.teacher_id,
                facility_id: "".to_string(), // Campo temporal
                enrolled: c.enrolled,
                semester: c.semester,
                academic_year: c.academic_year,
                active: c.active.unwrap_or(true),
            })
            .collect())
    }

    async fn get_courses_by_facility(&self, facility_id: &str) -> Result<Vec<Course>, String> {
        let courses = courses::Entity::find()
            .filter(courses::Column::FacilityId.eq(facility_id))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(courses
            .into_iter()
            .map(|c| Course {
                id: c.id,
                code: c.code,
                name: c.name,
                section: c.section,
                curriculum: sea_orm_active_enums::to_domain_curriculum(&c.curriculum),
                capacity: c.capacity,
                credits: c.credits,
                hours_per_week: c.hours_per_week,
                cycle: c.cycle,
                teacher_id: c.teacher_id,
                facility_id: "".to_string(), // Campo temporal
                enrolled: c.enrolled,
                semester: c.semester,
                academic_year: c.academic_year,
                active: c.active.unwrap_or(true),
            })
            .collect())
    }

    async fn get_courses_by_facility_name(
        &self,
        name_facility: &str,
    ) -> Result<Vec<Course>, String> {
        let courses = courses::Entity::find()
            .join(
                JoinType::InnerJoin,
                courses::Relation::CourseSchedules.def(),
            )
            .join(
                JoinType::InnerJoin,
                course_schedules::Relation::Facilities.def(),
            )
            .filter(facilities::Column::Name.eq(name_facility))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(courses
            .into_iter()
            .map(|c| Course {
                id: c.id,
                code: c.code,
                name: c.name,
                section: c.section,
                curriculum: sea_orm_active_enums::to_domain_curriculum(&c.curriculum),
                capacity: c.capacity,
                credits: c.credits,
                hours_per_week: c.hours_per_week,
                cycle: c.cycle,
                teacher_id: c.teacher_id,
                facility_id: "".to_string(),
                enrolled: c.enrolled,
                semester: c.semester,
                academic_year: c.academic_year,
                active: c.active.unwrap_or(true),
            })
            .collect())
    }

    async fn get_courses_by_schedule(&self, schedule_id: &str) -> Result<Course, String> {
        // Necesitarías un join con course_schedules
        let course_schedule = course_schedules::Entity::find_by_id(schedule_id)
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Horario no encontrado".to_string())?;

        let course_model = course_schedule
            .find_related(courses::Entity)
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Curso no encontrado para este horario".to_string())?;

        Ok(Course {
            id: course_model.id,
            code: course_model.code,
            name: course_model.name,
            section: course_model.section,
            curriculum: sea_orm_active_enums::to_domain_curriculum(&course_model.curriculum),
            capacity: course_model.capacity,
            credits: course_model.credits,
            hours_per_week: course_model.hours_per_week,
            cycle: course_model.cycle,
            teacher_id: course_model.teacher_id,
            facility_id: "".to_string(), // Campo temporal
            enrolled: course_model.enrolled,
            semester: course_model.semester,
            academic_year: course_model.academic_year,
            active: course_model.active.unwrap_or(true),
        })
    }

    async fn delete_course(&self, id: &str) -> Result<(), String> {
        let course = courses::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Curso no encontrado".to_string())?;

        let mut course: courses::ActiveModel = course.into();
        course.active = Set(Some(false)); // Borrado lógico
        course.updated_at = Set(Some(Utc::now().naive_utc()));

        course.update(&self.db).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
