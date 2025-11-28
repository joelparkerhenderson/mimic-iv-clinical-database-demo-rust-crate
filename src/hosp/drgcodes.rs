//! DRG Codes

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrgCodes {
    pub subject_id: Option<i64>,
    pub hadm_id: Option<i64>,
    pub drg_type: Option<String>,
    pub drg_code: Option<String>,
    pub description: Option<String>,
    pub drg_severity: Option<i64>,
    pub drg_mortality: Option<i64>,
}
