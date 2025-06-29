use serde::{Deserialize, Serialize};
use std::str::FromStr;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

impl FromStr for Curriculum {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Obligatory" => Ok(Curriculum::Obligatory),
            "Elective" => Ok(Curriculum::Elective),
            "Prerequisite" => Ok(Curriculum::Prerequisite),
            _ => Err(()),
        }
    }
}

impl ToString for Curriculum {
    fn to_string(&self) -> String {
        match self {
            Curriculum::Obligatory => "Obligatory".to_string(),
            Curriculum::Elective => "Elective".to_string(),
            Curriculum::Prerequisite => "Prerequisite".to_string(),
        }
    }
}

impl FromStr for EnrollmentStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "enrolled" => Ok(EnrollmentStatus::Enrolled),
            "withdrawn" => Ok(EnrollmentStatus::Withdrawn),
            "completed" => Ok(EnrollmentStatus::Completed),
            "failed" => Ok(EnrollmentStatus::Failed),
            "pending" => Ok(EnrollmentStatus::Pending),
            "dropped" => Ok(EnrollmentStatus::Dropped),
            _ => Err(()),
        }
    }
}

impl ToString for EnrollmentStatus {
    fn to_string(&self) -> String {
        match self {
            EnrollmentStatus::Enrolled => "enrolled".to_string(),
            EnrollmentStatus::Withdrawn => "withdrawn".to_string(),
            EnrollmentStatus::Completed => "completed".to_string(),
            EnrollmentStatus::Failed => "failed".to_string(),
            EnrollmentStatus::Pending => "pending".to_string(),
            EnrollmentStatus::Dropped => "dropped".to_string(),
        }
    }
}

impl FromStr for SessionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "theory" => Ok(SessionType::Theory),
            "laboratory" => Ok(SessionType::Laboratory),
            "seminar" => Ok(SessionType::Seminar),
            "practice" => Ok(SessionType::Practice),
            _ => Err(format!("Unknown session type: {}", s)),
        }
    }
}

impl ToString for SessionType {
    fn to_string(&self) -> String {
        match self {
            SessionType::Theory => "theory".to_string(),
            SessionType::Laboratory => "laboratory".to_string(),
            SessionType::Seminar => "seminar".to_string(),
            SessionType::Practice => "practice".to_string(),
        }
    }
}

impl FromStr for Weekday {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "monday" => Ok(Weekday::Monday),
            "tuesday" => Ok(Weekday::Tuesday),
            "wednesday" => Ok(Weekday::Wednesday),
            "thursday" => Ok(Weekday::Thursday),
            "friday" => Ok(Weekday::Friday),
            "saturday" => Ok(Weekday::Saturday),
            "sunday" => Ok(Weekday::Sunday),
            _ => Err(format!("Unknown weekday: {}", s)),
        }
    }
}

impl ToString for Weekday {
    fn to_string(&self) -> String {
        match self {
            Weekday::Monday => "monday".to_string(),
            Weekday::Tuesday => "tuesday".to_string(),
            Weekday::Wednesday => "wednesday".to_string(),
            Weekday::Thursday => "thursday".to_string(),
            Weekday::Friday => "friday".to_string(),
            Weekday::Saturday => "saturday".to_string(),
            Weekday::Sunday => "sunday".to_string(),
        }
    }
}

impl ToString for StudentStatus {
    fn to_string(&self) -> String {
        match self {
            StudentStatus::Regular => "regular".to_string(),
            StudentStatus::Observation => "observado".to_string(),
            StudentStatus::Graduated => "graduated".to_string(),
        }
    }
}

impl FromStr for StudentStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "regular" => Ok(StudentStatus::Regular),
            "observado" => Ok(StudentStatus::Observation),
            "graduated" => Ok(StudentStatus::Graduated),
            _ => Err(()),
        }
    }
}

impl ToString for ContractType {
    fn to_string(&self) -> String {
        match self {
            ContractType::Contracted => "contratado".to_string(),
            ContractType::Principal => "principal".to_string(),
            ContractType::Associate => "asociado".to_string(),
        }
    }
}

impl FromStr for ContractType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "contratado" => Ok(ContractType::Contracted),
            "principal" => Ok(ContractType::Principal),
            "asociado" => Ok(ContractType::Associate),
            _ => Err(()),
        }
    }
}
