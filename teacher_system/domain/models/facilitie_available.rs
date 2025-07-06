use crate::domain::models::enums::Weekday;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacilityAvailable {
    pub id: String,
    pub name: String,
    pub capacity: i32,
    pub facility_type: String,
    pub day: Weekday,
    pub hours_range: Vec<(u32, u32)>,
}
