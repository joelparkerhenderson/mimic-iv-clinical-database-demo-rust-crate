//! Transfers

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfers {
    pub subject_id: Option<i64>,
    pub hadm_id: Option<i64>,
    pub transfer_id: Option<i64>,
    pub eventtype: Option<String>,
    pub careunit: Option<String>,
    pub intime: Option<String>,
    pub outtime: Option<String>,
}
