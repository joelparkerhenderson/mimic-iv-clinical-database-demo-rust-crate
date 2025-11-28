//! D ICD Procedures

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DIcdProcedures {
    pub icd_code: Option<String>,
    pub icd_version: Option<i64>,
    pub long_title: Option<String>,
}
