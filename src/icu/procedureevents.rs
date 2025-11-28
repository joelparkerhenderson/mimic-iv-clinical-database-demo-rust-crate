//! Procedure Events

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureEvents {
    pub subject_id: Option<i64>,
    pub hadm_id: Option<i64>,
    pub stay_id: Option<i64>,
    pub caregiver_id: Option<i64>,
    pub starttime: Option<String>,
    pub endtime: Option<String>,
    pub storetime: Option<String>,
    pub itemid: Option<i64>,
    pub value: Option<f64>,
    pub valueuom: Option<String>,
    pub location: Option<String>,
    pub locationcategory: Option<String>,
    pub orderid: Option<i64>,
    pub linkorderid: Option<i64>,
    pub ordercategoryname: Option<String>,
    pub ordercategorydescription: Option<String>,
    pub patientweight: Option<f64>,
    pub isopenbag: Option<i64>,
    pub continueinnextdept: Option<i64>,
    pub statusdescription: Option<String>,
    #[serde(rename = "ORIGINALAMOUNT")]
    pub originalamount: Option<f64>,
    #[serde(rename = "ORIGINALRATE")]
    pub originalrate: Option<f64>,
}
