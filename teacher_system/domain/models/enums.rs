use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Curriculum {
    Obligatory,
    Elective,
    Prerequisite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnrollmentStatus {
    Enrolled,
    Withdrawn,
    Completed,
    Failed,
    Pending,
    Dropped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionType {
    Theory,
    Laboratory,
    Seminar,
    Practice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StudentStatus {
    Regular,
    Observation,
    Graduated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractType {
    Contracted,
    Principal,
    Associate,
}

impl Curriculum {
    pub fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "Obligatory" => Ok(Self::Obligatory),
            "Elective" => Ok(Self::Elective),
            "Prerequisite" => Ok(Self::Prerequisite),
            _ => Err(()),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Obligatory => "Obligatory".to_string(),
            Self::Elective => "Elective".to_string(),
            Self::Prerequisite => "Prerequisite".to_string(),
        }
    }
}

impl EnrollmentStatus {
    pub fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "enrolled" => Ok(Self::Enrolled),
            "withdrawn" => Ok(Self::Withdrawn),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "pending" => Ok(Self::Pending),
            "dropped" => Ok(Self::Dropped),
            _ => Err(()),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Enrolled => "enrolled".to_string(),
            Self::Withdrawn => "withdrawn".to_string(),
            Self::Completed => "completed".to_string(),
            Self::Failed => "failed".to_string(),
            Self::Pending => "pending".to_string(),
            Self::Dropped => "dropped".to_string(),
        }
    }
}

impl SessionType {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "theory" => Ok(Self::Theory),
            "laboratory" => Ok(Self::Laboratory),
            "seminar" => Ok(Self::Seminar),
            "practice" => Ok(Self::Practice),
            _ => Err(format!("Unknown session type: {}", s)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Theory => "theory".to_string(),
            Self::Laboratory => "laboratory".to_string(),
            Self::Seminar => "seminar".to_string(),
            Self::Practice => "practice".to_string(),
        }
    }
}

impl Weekday {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "monday" => Ok(Self::Monday),
            "tuesday" => Ok(Self::Tuesday),
            "wednesday" => Ok(Self::Wednesday),
            "thursday" => Ok(Self::Thursday),
            "friday" => Ok(Self::Friday),
            "saturday" => Ok(Self::Saturday),
            "sunday" => Ok(Self::Sunday),
            _ => Err(format!("Unknown weekday: {}", s)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Monday => "monday".to_string(),
            Self::Tuesday => "tuesday".to_string(),
            Self::Wednesday => "wednesday".to_string(),
            Self::Thursday => "thursday".to_string(),
            Self::Friday => "friday".to_string(),
            Self::Saturday => "saturday".to_string(),
            Self::Sunday => "sunday".to_string(),
        }
    }
}

impl StudentStatus {
    pub fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "regular" => Ok(Self::Regular),
            "observado" => Ok(Self::Observation),
            "graduated" => Ok(Self::Graduated),
            _ => Err(()),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Regular => "regular".to_string(),
            Self::Observation => "observado".to_string(),
            Self::Graduated => "graduated".to_string(),
        }
    }
}

impl ContractType {
    pub fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "contratado" => Ok(Self::Contracted),
            "principal" => Ok(Self::Principal),
            "asociado" => Ok(Self::Associate),
            _ => Err(()),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Contracted => "contratado".to_string(),
            Self::Principal => "principal".to_string(),
            Self::Associate => "asociado".to_string(),
        }
    }
}
