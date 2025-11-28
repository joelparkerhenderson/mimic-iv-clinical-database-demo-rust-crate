//! POE

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poe {
    pub poe_id: Option<i64>,
    pub poe_seq: Option<i64>,
    pub subject_id: Option<i64>,
    pub hadm_id: Option<i64>,
    pub ordertime: Option<String>,
    pub order_type: Option<String>,
    pub order_subtype: Option<String>,
    pub transaction_type: Option<String>,
    pub discontinue_of_poe_id: Option<i64>,
    pub discontinued_by_poe_id: Option<i64>,
    pub order_provider_id: Option<String>,
    pub order_status: Option<String>,
}
