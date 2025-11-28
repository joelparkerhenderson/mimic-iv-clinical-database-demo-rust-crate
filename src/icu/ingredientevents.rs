//! Ingredient Events

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngredientEvents {
    pub subject_id: Option<i64>,
    pub hadm_id: Option<i64>,
    pub stay_id: Option<i64>,
    pub caregiver_id: Option<i64>,
    pub starttime: Option<String>,
    pub endtime: Option<String>,
    pub storetime: Option<String>,
    pub itemid: Option<i64>,
    pub amount: Option<f64>,
    pub amountuom: Option<String>,
    pub rate: Option<f64>,
    pub rateuom: Option<String>,
    pub orderid: Option<i64>,
    pub linkorderid: Option<i64>,
    pub statusdescription: Option<String>,
    pub originalamount: Option<f64>,
    pub originalrate: Option<f64>,
}
