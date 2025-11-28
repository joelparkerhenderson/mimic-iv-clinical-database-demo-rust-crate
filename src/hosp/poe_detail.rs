//! POE Detail

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoeDetail {
    pub poe_id: Option<i64>,
    pub poe_seq: Option<i64>,
    pub subject_id: Option<i64>,
    pub field_name: Option<String>,
    pub field_value: Option<String>,
}
