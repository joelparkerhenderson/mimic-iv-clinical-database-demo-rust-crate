//! Diagnoses ICD

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosesIcd {
    pub subject_id: Option<i64>,
    pub hadm_id: Option<i64>,
    pub seq_num: Option<i64>,
    pub icd_code: Option<String>,
    pub icd_version: Option<i64>,
}
