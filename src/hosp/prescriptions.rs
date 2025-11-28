//! Prescriptions

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prescriptions {
    pub subject_id: Option<i64>,
    pub hadm_id: Option<i64>,
    pub pharmacy_id: Option<i64>,
    pub poe_id: Option<i64>,
    pub poe_seq: Option<i64>,
    pub order_provider_id: Option<String>,
    pub starttime: Option<String>,
    pub stoptime: Option<String>,
    pub drug_type: Option<String>,
    pub drug: Option<String>,
    pub formulary_drug_cd: Option<String>,
    pub gsn: Option<String>,
    pub ndc: Option<String>,
    pub prod_strength: Option<String>,
    pub form_rx: Option<String>,
    pub dose_val_rx: Option<String>,
    pub dose_unit_rx: Option<String>,
    pub form_val_disp: Option<String>,
    pub form_unit_disp: Option<String>,
    pub doses_per_24_hrs: Option<f64>,
    pub route: Option<String>,
}
