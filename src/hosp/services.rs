//! Services

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Services {
    pub subject_id: Option<i64>,
    pub hadm_id: Option<i64>,
    pub transfertime: Option<String>,
    pub prev_service: Option<String>,
    pub curr_service: Option<String>,
}
