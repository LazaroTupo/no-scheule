use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Facility {
    pub id: String,
    pub name: String,
    pub capacity: i32,
    pub facility_type: String,
    pub created_at: Option<String>,
}
