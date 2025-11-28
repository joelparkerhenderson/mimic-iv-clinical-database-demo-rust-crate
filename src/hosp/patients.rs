//! Patients

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patients {
    pub subject_id: Option<i64>,
    pub gender: Option<String>,
    pub anchor_age: Option<i64>,
    pub anchor_year: Option<i64>,
    pub anchor_year_group: Option<String>,
    pub dod: Option<String>,
}
