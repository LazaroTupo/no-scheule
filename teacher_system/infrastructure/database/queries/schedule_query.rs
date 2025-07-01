use crate::domain::{
    models::enums::Weekday, models::schedule::Schedule,
    repositories::schedule_repository::ScheduleRepository,
};
use crate::infrastructure::database::{
    conexion,
    entities::{course_schedules, courses, facilities, sea_orm_active_enums},
};
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
    Set,
};
use std::str::FromStr;

#[derive(Clone)]
pub struct SupabaseScheduleRepository;

impl SupabaseScheduleRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ScheduleRepository for SupabaseScheduleRepository {
    async fn create_schedule(&self, schedule: &Schedule) -> Result<(), String> {
        let db = conexion::get_conn();

        let schedule_model = course_schedules::ActiveModel {
            id: Set(schedule.id.clone()),
            course_id: Set(schedule.course_id.clone()),
            day: Set(sea_orm_active_enums::to_db_daytype(&schedule.day)),
            start_time: Set(schedule.start_time),
            end_time: Set(schedule.end_time),
            session_type: Set(sea_orm_active_enums::to_db_session(&schedule.session_type)),
            location_detail: Set(schedule.location_detail.clone()),
            facility_id: Set(schedule.facility_id.clone()),
            created_at: Set(Some(Utc::now().naive_utc())),
        };

        schedule_model.insert(db).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn update_schedule(&self, schedule: &Schedule) -> Result<(), String> {
        let db = conexion::get_conn();

        let mut schedule_model: course_schedules::ActiveModel =
            course_schedules::Entity::find_by_id(&schedule.id)
                .one(db)
                .await
                .map_err(|e| e.to_string())?
                .ok_or("Schedule not found")?
                .into();

        schedule_model.course_id = Set(schedule.course_id.clone());
        schedule_model.day = Set(sea_orm_active_enums::to_db_daytype(&schedule.day));
        schedule_model.start_time = Set(schedule.start_time);
        schedule_model.end_time = Set(schedule.end_time);
        schedule_model.session_type =
            Set(sea_orm_active_enums::to_db_session(&schedule.session_type));
        schedule_model.location_detail = Set(schedule.location_detail.clone());
        schedule_model.facility_id = Set(schedule.facility_id.clone());

        schedule_model.update(db).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_schedule_by_id(&self, schedule_id: &str) -> Result<Option<Schedule>, String> {
        let db = conexion::get_conn();

        let course_schedule = course_schedules::Entity::find_by_id(schedule_id)
            .one(db)
            .await
            .map_err(|e| e.to_string())?
            .map(|s| Schedule {
                id: s.id,
                course_id: s.course_id,
                day: sea_orm_active_enums::to_domain_weekday(&s.day),
                start_time: s.start_time,
                end_time: s.end_time,
                session_type: sea_orm_active_enums::to_domain_session(&s.session_type),
                location_detail: s.location_detail,
                created_at: s.created_at.map(|dt| dt.to_string()),
                facility_id: s.facility_id,
            });

        Ok(course_schedule)
    }

    async fn get_all_schedules(&self) -> Result<Vec<Schedule>, String> {
        let db = conexion::get_conn();

        let schedules = course_schedules::Entity::find()
            .all(db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|s| Schedule {
                id: s.id,
                course_id: s.course_id,
                day: sea_orm_active_enums::to_domain_weekday(&s.day),
                start_time: s.start_time,
                end_time: s.end_time,
                session_type: sea_orm_active_enums::to_domain_session(&s.session_type),
                location_detail: s.location_detail,
                created_at: s.created_at.map(|dt| dt.to_string()),
                facility_id: s.facility_id,
            })
            .collect();

        Ok(schedules)
    }

    async fn get_schedules_by_course(&self, course_id: &str) -> Result<Option<Schedule>, String> {
        let db = conexion::get_conn();

        let schedules = course_schedules::Entity::find()
            .filter(course_schedules::Column::CourseId.eq(course_id))
            .all(db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|s| Schedule {
                id: s.id,
                course_id: s.course_id,
                day: sea_orm_active_enums::to_domain_weekday(&s.day),
                start_time: s.start_time,
                end_time: s.end_time,
                session_type: sea_orm_active_enums::to_domain_session(&s.session_type),
                location_detail: s.location_detail,
                created_at: s.created_at.map(|dt| dt.to_string()),
                facility_id: s.facility_id,
            })
            .next();

        Ok(schedules)
    }

    async fn get_schedules_by_course_name(
        &self,
        name_course: &str,
    ) -> Result<Vec<Schedule>, String> {
        let db = conexion::get_conn();

        let schedules = course_schedules::Entity::find()
            .join(
                JoinType::InnerJoin,
                course_schedules::Relation::Courses.def(),
            )
            .filter(courses::Column::Name.eq(name_course))
            .all(db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|s| Schedule {
                id: s.id,
                course_id: s.course_id,
                day: sea_orm_active_enums::to_domain_weekday(&s.day),
                start_time: s.start_time,
                end_time: s.end_time,
                session_type: sea_orm_active_enums::to_domain_session(&s.session_type),
                location_detail: s.location_detail,
                created_at: s.created_at.map(|dt| dt.to_string()),
                facility_id: s.facility_id,
            })
            .collect();

        Ok(schedules)
    }

    async fn get_schedules_by_facility(&self, facility_id: &str) -> Result<Vec<Schedule>, String> {
        let db = conexion::get_conn();

        let schedules = course_schedules::Entity::find()
            .filter(course_schedules::Column::FacilityId.eq(facility_id))
            .all(db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|s| Schedule {
                id: s.id,
                course_id: s.course_id,
                day: sea_orm_active_enums::to_domain_weekday(&s.day),
                start_time: s.start_time,
                end_time: s.end_time,
                session_type: sea_orm_active_enums::to_domain_session(&s.session_type),
                location_detail: s.location_detail,
                created_at: s.created_at.map(|dt| dt.to_string()),
                facility_id: s.facility_id,
            })
            .collect();

        Ok(schedules)
    }

    async fn get_schedules_by_facility_name(
        &self,
        name_facility: &str,
    ) -> Result<Vec<Schedule>, String> {
        let db = conexion::get_conn();

        let schedules = course_schedules::Entity::find()
            .join(
                JoinType::InnerJoin,
                course_schedules::Relation::Facilities.def(),
            )
            .filter(facilities::Column::Name.eq(name_facility))
            .all(db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|s| Schedule {
                id: s.id,
                course_id: s.course_id,
                day: sea_orm_active_enums::to_domain_weekday(&s.day),
                start_time: s.start_time,
                end_time: s.end_time,
                session_type: sea_orm_active_enums::to_domain_session(&s.session_type),
                location_detail: s.location_detail,
                created_at: s.created_at.map(|dt| dt.to_string()),
                facility_id: s.facility_id,
            })
            .collect();

        Ok(schedules)
    }

    async fn get_schedules_by_user(&self, user_id: &str) -> Result<Vec<Schedule>, String> {
        let db = conexion::get_conn();

        let schedules = course_schedules::Entity::find()
            .join(
                JoinType::InnerJoin,
                course_schedules::Relation::Courses.def(),
            )
            .filter(courses::Column::TeacherId.eq(user_id))
            .all(db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|s| Schedule {
                id: s.id,
                course_id: s.course_id,
                day: sea_orm_active_enums::to_domain_weekday(&s.day),
                start_time: s.start_time,
                end_time: s.end_time,
                session_type: sea_orm_active_enums::to_domain_session(&s.session_type),
                location_detail: s.location_detail,
                created_at: s.created_at.map(|dt| dt.to_string()),
                facility_id: s.facility_id,
            })
            .collect();

        Ok(schedules)
    }

    async fn get_schedules_by_weekday(&self, weekday: &str) -> Result<Vec<Schedule>, String> {
        let db = conexion::get_conn();

        let day_enum = sea_orm_active_enums::to_db_daytype(&Weekday::from_str(weekday).unwrap());

        let schedule = course_schedules::Entity::find()
            .filter(course_schedules::Column::Day.eq(day_enum))
            .all(db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|s| Schedule {
                id: s.id,
                course_id: s.course_id,
                day: sea_orm_active_enums::to_domain_weekday(&s.day),
                start_time: s.start_time,
                end_time: s.end_time,
                session_type: sea_orm_active_enums::to_domain_session(&s.session_type),
                location_detail: s.location_detail,
                created_at: s.created_at.map(|dt| dt.to_string()),
                facility_id: s.facility_id,
            })
            .collect();

        Ok(schedule)
    }

    async fn delete_schedule(&self, schedule_id: &str) -> Result<(), String> {
        let db = conexion::get_conn();

        course_schedules::Entity::delete_by_id(schedule_id)
            .exec(db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
