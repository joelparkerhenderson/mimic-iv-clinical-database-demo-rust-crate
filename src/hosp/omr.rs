//! OMR

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Omr {
    pub subject_id: Option<i64>,
    pub chartdate: Option<String>,
    pub seq_num: Option<i64>,
    pub result_name: Option<String>,
    pub result_value: Option<String>,
}
