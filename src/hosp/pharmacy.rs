//! Pharmacy

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pharmacy {
    pub subject_id: Option<i64>,
    pub hadm_id: Option<i64>,
    pub pharmacy_id: Option<i64>,
    pub poe_id: Option<i64>,
    pub starttime: Option<String>,
    pub stoptime: Option<String>,
    pub medication: Option<String>,
    pub proc_type: Option<String>,
    pub status: Option<String>,
    pub entertime: Option<String>,
    pub verifiedtime: Option<String>,
    pub route: Option<String>,
    pub frequency: Option<String>,
    pub disp_sched: Option<String>,
    pub infusion_type: Option<String>,
    pub sliding_scale: Option<String>,
    pub lockout_interval: Option<String>,
    pub basal_rate: Option<String>,
    pub one_hr_max: Option<String>,
    pub doses_per_24_hrs: Option<f64>,
    pub duration: Option<f64>,
    pub duration_interval: Option<String>,
    pub expiration_value: Option<i64>,
    pub expiration_unit: Option<String>,
    pub expirationdate: Option<String>,
    pub dispensation: Option<String>,
    pub fill_quantity: Option<String>,
}
