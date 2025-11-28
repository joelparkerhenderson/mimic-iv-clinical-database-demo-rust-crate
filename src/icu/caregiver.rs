//! Caregiver

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Caregiver {
    pub caregiver_id: Option<i64>,
}
