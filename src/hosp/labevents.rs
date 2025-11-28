//! Lab Events

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabEvents {
    pub labevent_id: Option<i64>,
    pub subject_id: Option<i64>,
    pub hadm_id: Option<i64>,
    pub specimen_id: Option<i64>,
    pub itemid: Option<i64>,
    pub order_provider_id: Option<String>,
    pub charttime: Option<String>,
    pub storetime: Option<String>,
    pub value: Option<String>,
    pub valuenum: Option<f64>,
    pub valueuom: Option<String>,
    pub ref_range_lower: Option<f64>,
    pub ref_range_upper: Option<f64>,
    pub flag: Option<String>,
    pub priority: Option<String>,
    pub comments: Option<String>,
}
