//! HCPCS Events

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HcpcsEvents {
    pub subject_id: Option<i64>,
    pub hadm_id: Option<i64>,
    pub chartdate: Option<String>,
    pub hcpcs_cd: Option<String>,
    pub seq_num: Option<i64>,
    pub short_description: Option<String>,
}
