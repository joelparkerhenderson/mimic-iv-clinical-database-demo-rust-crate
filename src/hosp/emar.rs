//! EMAR

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emar {
    pub subject_id: Option<i64>,
    pub hadm_id: Option<i64>,
    pub emar_id: Option<String>,
    pub emar_seq: Option<i64>,
    pub poe_id: Option<i64>,
    pub pharmacy_id: Option<i64>,
    pub enter_provider_id: Option<String>,
    pub charttime: Option<String>,
    pub medication: Option<String>,
    pub event_txt: Option<String>,
    pub scheduletime: Option<String>,
    pub storetime: Option<String>,
}
