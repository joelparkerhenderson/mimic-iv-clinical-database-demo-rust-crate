//! D Lab Items

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DLabItems {
    pub itemid: Option<i64>,
    pub label: Option<String>,
    pub fluid: Option<String>,
    pub category: Option<String>,
}
