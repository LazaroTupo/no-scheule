use crate::domain::{
    models::facilitie::Facility, repositories::facility_repository::FacilityRepository,
};
use crate::infrastructure::database::{
    conexion,
    entities::{course_schedules, courses, facilities, users},
};
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
    Set,
};

#[derive(Clone)]
pub struct SupabaseFacilityRepository;

impl SupabaseFacilityRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl FacilityRepository for SupabaseFacilityRepository {
    async fn create_facility(&self, facility: &Facility) -> Result<(), String> {
        let db = conexion::get_conn();

        let new_facility = facilities::ActiveModel {
            id: Set(facility.id.clone()),
            name: Set(facility.name.clone()),
            capacity: Set(Some(facility.capacity.clone())),
            facility_type: Set(Some(facility.facility_type.clone())),
            created_at: Set(Some(Utc::now().naive_utc())),
        };

        new_facility.insert(db).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn update_facility(&self, facility: &Facility) -> Result<(), String> {
        let db = conexion::get_conn();

        let existing = facilities::Entity::find_by_id(&facility.id)
            .one(db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Facility not found".to_string())?;

        let mut model: facilities::ActiveModel = existing.into();
        model.name = Set(facility.name.clone());
        model.capacity = Set(Some(facility.capacity.clone()));
        model.facility_type = Set(Some(facility.facility_type.clone()));

        model.update(db).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_facility_by_id(&self, id: &str) -> Result<Facility, String> {
        let db = conexion::get_conn();

        facilities::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| e.to_string())?
            .map(|model| Facility {
                id: model.id,
                name: model.name,
                capacity: model.capacity.unwrap_or_default(),
                facility_type: model.facility_type.unwrap_or_default(),
                created_at: model.created_at.map(|dt| dt.to_string()),
            })
            .ok_or_else(|| "Facility not found".to_string())
    }

    async fn get_all_facilities(&self) -> Result<Vec<Facility>, String> {
        let db = conexion::get_conn();

        facilities::Entity::find()
            .all(db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|model| {
                Ok(Facility {
                    id: model.id,
                    name: model.name,
                    capacity: model.capacity.unwrap_or_default(),
                    facility_type: model.facility_type.unwrap_or_default(),
                    created_at: model.created_at.map(|dt| dt.to_string()),
                })
            })
            .collect()
    }

    async fn get_facilities_by_course(&self, course_id: &str) -> Result<Facility, String> {
        let db = conexion::get_conn();

        let schedules = course_schedules::Entity::find()
            .filter(course_schedules::Column::CourseId.eq(course_id))
            .find_also_related(facilities::Entity)
            .one(db)
            .await
            .map_err(|e| e.to_string())?;

        let facilities = schedules
            .into_iter()
            .filter_map(|(_, facility_opt)| facility_opt)
            .next()
            .map(|model| Facility {
                id: model.id,
                name: model.name,
                capacity: model.capacity.unwrap_or_default(),
                facility_type: model.facility_type.unwrap_or_default(),
                created_at: model.created_at.map(|dt| dt.to_string()),
            })
            .ok_or_else(|| "No facility found for this course".to_string())?;

        Ok(facilities)
    }

    async fn get_facilities_name_course(&self, name_course: &str) -> Result<Vec<Facility>, String> {
        let db = conexion::get_conn();

        let facilities = facilities::Entity::find()
            .join(
                JoinType::InnerJoin,
                facilities::Relation::CourseSchedules.def(),
            )
            .join(
                JoinType::InnerJoin,
                course_schedules::Relation::Courses.def(),
            )
            .filter(courses::Column::Name.eq(name_course))
            .all(db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|model| Facility {
                id: model.id,
                name: model.name,
                capacity: model.capacity.unwrap_or_default(),
                facility_type: model.facility_type.unwrap_or_default(),
                created_at: model.created_at.map(|dt| dt.to_string()),
            })
            .collect();

        Ok(facilities)
    }

    async fn get_facilities_by_schedule(&self, schedule_id: &str) -> Result<Facility, String> {
        let db = conexion::get_conn();

        course_schedules::Entity::find_by_id(schedule_id)
            .find_also_related(facilities::Entity)
            .one(db)
            .await
            .map_err(|e| e.to_string())?
            .and_then(|(_, facility)| facility)
            .map(|model| Facility {
                id: model.id,
                name: model.name,
                capacity: model.capacity.unwrap_or_default(),
                facility_type: model.facility_type.unwrap_or_default(),
                created_at: model.created_at.map(|dt| dt.to_string()),
            })
            .ok_or_else(|| "Facility not found for this schedule".to_string())
    }

    async fn get_facilities_by_user(&self, user_id: &str) -> Result<Vec<Facility>, String> {
        let db = conexion::get_conn();

        let facilitie = facilities::Entity::find()
            .join(
                JoinType::InnerJoin,
                facilities::Relation::CourseSchedules.def(),
            )
            .join(JoinType::InnerJoin, courses::Relation::Users.def())
            .filter(users::Column::Id.eq(user_id))
            .all(db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|model| Facility {
                id: model.id,
                name: model.name,
                capacity: model.capacity.unwrap_or_default(),
                facility_type: model.facility_type.unwrap_or_default(),
                created_at: model.created_at.map(|dt| dt.to_string()),
            })
            .collect();

        Ok(facilitie)
    }

    async fn delete_facility(&self, id: &str) -> Result<(), String> {
        let db = conexion::get_conn();

        facilities::Entity::delete_by_id(id)
            .exec(db)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
