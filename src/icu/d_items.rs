//! D Items

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DItems {
    pub itemid: Option<i64>,
    pub label: Option<String>,
    pub abbreviation: Option<String>,
    pub linksto: Option<String>,
    pub category: Option<String>,
    pub unitname: Option<String>,
    pub param_type: Option<String>,
    pub lownormalvalue: Option<f64>,
    pub highnormalvalue: Option<f64>,
}
