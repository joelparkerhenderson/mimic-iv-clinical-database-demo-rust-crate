//! D HCPCS

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DHcpcs {
    pub code: Option<String>,
    pub category: Option<i64>,
    pub long_description: Option<String>,
    pub short_description: Option<String>,
}
