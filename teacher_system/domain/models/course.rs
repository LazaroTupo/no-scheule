use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Curriculum {
    Engineering,
    Science,
    Humanities,
    // otros
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: String,
    pub code: String,
    pub name: String,
    pub section: i32,
    pub curriculum: Curriculum,
    pub capacity: i32,
    pub credits: i32,
    pub hours_per_week: i32,
    pub cycle: i32,
    pub teacher_id: String,
    pub schedule_id: String,
    pub enrolled: i32,
    pub semester: String,
    pub academic_year: i32,
    pub active: bool,
}

impl Course {
    pub fn available_spots(&self) -> i32 {
        self.capacity - self.enrolled
    }
}
