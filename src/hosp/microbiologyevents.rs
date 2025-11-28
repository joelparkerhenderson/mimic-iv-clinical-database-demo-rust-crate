//! Microbiology Events

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicrobiologyEvents {
    pub microevent_id: Option<i64>,
    pub subject_id: Option<i64>,
    pub hadm_id: Option<i64>,
    pub micro_specimen_id: Option<i64>,
    pub order_provider_id: Option<String>,
    pub chartdate: Option<String>,
    pub charttime: Option<String>,
    pub spec_itemid: Option<i64>,
    pub spec_type_desc: Option<String>,
    pub test_seq: Option<i64>,
    pub storedate: Option<String>,
    pub storetime: Option<String>,
    pub test_itemid: Option<i64>,
    pub test_name: Option<String>,
    pub org_itemid: Option<i64>,
    pub org_name: Option<String>,
    pub isolate_num: Option<i64>,
    pub quantity: Option<String>,
    pub ab_itemid: Option<i64>,
    pub ab_name: Option<String>,
    pub dilution_text: Option<String>,
    pub dilution_comparison: Option<String>,
    pub dilution_value: Option<f64>,
    pub interpretation: Option<String>,
    pub comments: Option<String>,
}
