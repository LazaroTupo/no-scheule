use crate::domain::models::schedule::{SessionType, Weekday};
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleDTO {
    pub day: Weekday,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub session_type: SessionType,
    pub facility_id: Option<String>,
    pub location_detail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleResponseDTO {
    pub id: String,
    pub facility_name: String,
    pub day: Weekday,
    pub start_time: String,
    pub end_time: String,
    pub session_type: SessionType,
    pub location_detail: Option<String>,
}
