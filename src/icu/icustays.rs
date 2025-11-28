//! ICU Stays

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcuStays {
    pub subject_id: Option<i64>,
    pub hadm_id: Option<i64>,
    pub stay_id: Option<i64>,
    pub first_careunit: Option<String>,
    pub last_careunit: Option<String>,
    pub intime: Option<String>,
    pub outtime: Option<String>,
    pub los: Option<f64>,
}
