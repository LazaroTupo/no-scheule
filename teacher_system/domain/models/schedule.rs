use crate::domain::models::enums::{SessionType, Weekday};
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub id: String,
    pub course_id: String,
    pub day: Weekday,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub session_type: SessionType,
    pub location_detail: Option<String>,
    pub created_at: Option<String>,
    pub facility_id: String,
}

impl Schedule {
    pub fn conflicts_with(&self, other: &Schedule) -> bool {
        self.day == other.day
            && self.start_time < other.end_time
            && self.end_time > other.start_time
    }
}
