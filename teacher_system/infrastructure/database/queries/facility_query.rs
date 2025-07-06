use crate::domain::{
    models::enums::Weekday, models::facilitie::Facility,
    models::facilitie_available::FacilityAvailable,
    repositories::facility_repository::FacilityRepository,
};
use crate::infrastructure::database::entities::{
    course_schedules, courses, facilities, sea_orm_active_enums, users,
};
use async_trait::async_trait;
use chrono::NaiveTime;
use chrono::Timelike;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter,
    QuerySelect, RelationTrait, Set,
};
use shared::config::connect_to_supabase;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct SupabaseFacilityRepository {
    db: DatabaseConnection,
}

impl SupabaseFacilityRepository {
    pub async fn new() -> Result<Self, String> {
        let db = connect_to_supabase().await.map_err(|e| e.to_string())?;
        Ok(Self { db })
    }
}

#[async_trait]
impl FacilityRepository for SupabaseFacilityRepository {
    async fn create_facility(&self, facility: &Facility) -> Result<(), String> {
        let new_facility = facilities::ActiveModel {
            id: Set(facility.id.clone()),
            name: Set(facility.name.clone()),
            capacity: Set(Some(facility.capacity.clone())),
            facility_type: Set(Some(facility.facility_type.clone())),
            created_at: Set(Some(Utc::now().naive_utc())),
        };

        new_facility
            .insert(&self.db)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn update_facility(&self, facility: &Facility) -> Result<(), String> {
        let existing = facilities::Entity::find_by_id(&facility.id)
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Facility not found".to_string())?;

        let mut model: facilities::ActiveModel = existing.into();
        model.name = Set(facility.name.clone());
        model.capacity = Set(Some(facility.capacity.clone()));
        model.facility_type = Set(Some(facility.facility_type.clone()));

        model.update(&self.db).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_facility_by_id(&self, id: &str) -> Result<Facility, String> {
        facilities::Entity::find_by_id(id)
            .one(&self.db)
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
        facilities::Entity::find()
            .all(&self.db)
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
        let schedules = course_schedules::Entity::find()
            .filter(course_schedules::Column::CourseId.eq(course_id))
            .find_also_related(facilities::Entity)
            .one(&self.db)
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
            .all(&self.db)
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
        course_schedules::Entity::find_by_id(schedule_id)
            .find_also_related(facilities::Entity)
            .one(&self.db)
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
        let facilitie = facilities::Entity::find()
            .join(
                JoinType::InnerJoin,
                facilities::Relation::CourseSchedules.def(),
            )
            .join(JoinType::InnerJoin, courses::Relation::Users.def())
            .filter(users::Column::Id.eq(user_id))
            .all(&self.db)
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
        facilities::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_facility_available(&self) -> Result<Vec<FacilityAvailable>, String> {
        // Obtener todas las facilities
        let facilities = facilities::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        // Obtener todos los horarios ocupados agrupados por facility_id y día
        let occupied_schedules = course_schedules::Entity::find()
            .find_also_related(facilities::Entity)
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        // Procesar los resultados
        let mut available_facilities = Vec::new();

        for facility in facilities {
            // Filtrar los horarios ocupados para esta facility
            let occupied_for_facility: Vec<&course_schedules::Model> = occupied_schedules
                .iter()
                .filter(|(schedule, _)| schedule.facility_id == facility.id)
                .map(|(schedule, _)| schedule)
                .collect();

            // Agrupar por día
            let mut schedules_by_day: BTreeMap<Weekday, Vec<(NaiveTime, NaiveTime)>> =
                BTreeMap::new();

            let occupied_for_facility_is_empty = occupied_for_facility.is_empty();

            for schedule in occupied_for_facility {
                let day = sea_orm_active_enums::to_domain_weekday(&schedule.day);
                schedules_by_day
                    .entry(day)
                    .or_default()
                    .push((schedule.start_time, schedule.end_time));
            }

            // Para cada día, calcular los rangos disponibles
            for (day, occupied_ranges) in schedules_by_day {
                // Si no hay horarios ocupados, toda la jornada está disponible
                if occupied_ranges.is_empty() {
                    available_facilities.push(FacilityAvailable {
                        id: facility.id.clone(),
                        name: facility.name.clone(),
                        capacity: facility.capacity.unwrap_or(0),
                        facility_type: facility.facility_type.as_ref().cloned().unwrap_or_default(),
                        day,
                        hours_range: vec![(0, 23)],
                    });
                    continue;
                }

                // Ordenar los rangos ocupados por hora de inicio
                let mut occupied_ranges = occupied_ranges;
                occupied_ranges.sort_by(|a, b| a.0.cmp(&b.0));

                // Calcular los rangos disponibles (inversos de los ocupados)
                let mut available_ranges = Vec::new();
                let mut last_end = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

                for (start, end) in occupied_ranges {
                    if start > last_end {
                        available_ranges.push((last_end.hour(), start.hour()));
                    }
                    if end > last_end {
                        last_end = end;
                    }
                }

                // Agregar el último rango disponible si queda tiempo
                if last_end < NaiveTime::from_hms_opt(23, 59, 59).unwrap() {
                    available_ranges.push((last_end.hour(), 23));
                }

                // Crear el registro de disponibilidad
                available_facilities.push(FacilityAvailable {
                    id: facility.id.clone(),
                    name: facility.name.clone(),
                    capacity: facility.capacity.unwrap_or(0),
                    facility_type: facility.facility_type.as_ref().cloned().unwrap_or_default(),
                    day,
                    hours_range: available_ranges,
                });
            }

            // Si no hay horarios ocupados para ninguna facility, agregar todos los días como disponibles
            if occupied_for_facility_is_empty {
                for day in [
                    Weekday::Monday,
                    Weekday::Tuesday,
                    Weekday::Wednesday,
                    Weekday::Thursday,
                    Weekday::Friday,
                    Weekday::Saturday,
                    Weekday::Sunday,
                ] {
                    available_facilities.push(FacilityAvailable {
                        id: facility.id.clone(),
                        name: facility.name.clone(),
                        capacity: facility.capacity.unwrap_or(0),
                        facility_type: facility.facility_type.as_ref().cloned().unwrap_or_default(),
                        day,
                        hours_range: vec![(0, 23)], // Todo el día disponible
                    });
                }
            }
        }

        Ok(available_facilities)
    }
}
