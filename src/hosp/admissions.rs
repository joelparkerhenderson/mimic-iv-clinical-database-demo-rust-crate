//! Admissions

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Admissions {
    pub subject_id: Option<i64>,
    pub hadm_id: Option<i64>,
    pub admittime: Option<String>,
    pub dischtime: Option<String>,
    pub deathtime: Option<String>,
    pub admission_type: Option<String>,
    pub admit_provider_id: Option<String>,
    pub admission_location: Option<String>,
    pub discharge_location: Option<String>,
    pub insurance: Option<String>,
    pub language: Option<String>,
    pub marital_status: Option<String>,
    pub race: Option<String>,
    pub edregtime: Option<String>,
    pub edouttime: Option<String>,
    pub hospital_expire_flag: Option<i64>,
}
