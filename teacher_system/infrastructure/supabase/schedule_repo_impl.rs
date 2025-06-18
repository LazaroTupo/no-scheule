use crate::domain::{
    models::{Schedule, Weekday},
    repositories::ScheduleRepository,
};
use async_trait::async_trait;
use supabase_rs::SupabaseClient;

pub struct SupabaseScheduleRepository {
    client: SupabaseClient,
}

impl SupabaseScheduleRepository {
    pub fn new(client: SupabaseClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl ScheduleRepository for SupabaseScheduleRepository {
    async fn create_schedule(&self, schedule: &Schedule) -> Result<(), String> {
        let response = self
            .client
            .from("course_schedules")
            .insert(json!({
                "id": schedule.id,
                "facilitie_id": schedule.facility_id,
                "day": schedule.day,
                "start_time": schedule.start_time,
                "end_time": schedule.end_time,
                "session_type": schedule.session_type,
                "location_detail": schedule.location_detail,
            }))
            .execute()
            .await
            .map_err(|e| e.to_string())?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err("Error al crear horario".into())
        }
    }

    // Implementar otros métodos
}
