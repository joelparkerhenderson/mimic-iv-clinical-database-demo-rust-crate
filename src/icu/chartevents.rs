//! Chart Events

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartEvents {
    pub subject_id: Option<i64>,
    pub hadm_id: Option<i64>,
    pub stay_id: Option<i64>,
    pub caregiver_id: Option<i64>,
    pub charttime: Option<String>,
    pub storetime: Option<String>,
    pub itemid: Option<i64>,
    pub value: Option<String>,
    pub valuenum: Option<f64>,
    pub valueuom: Option<String>,
    pub warning: Option<i64>,
}
