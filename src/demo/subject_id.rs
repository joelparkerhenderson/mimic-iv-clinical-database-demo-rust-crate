//! Demo Subject ID

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemoSubjectId {
    pub subject_id: Option<i64>,
}
