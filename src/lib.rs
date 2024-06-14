use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


pub struct Both {
    before: Before,
    after: After,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Before {
    pub p1: Option<f64>,
    pub p2: Option<f64>,
    pub p3: Option<f64>,
    pub p4: Option<f64>,
    pub p5: Option<f64>,
    pub p6: Option<f64>,
    pub p7: Option<f64>,
    pub p8: Option<f64>,
    pub p9: Option<f64>,
}

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct After {
    #[serde_as(deserialize_as = "Option<DisplayFromStr>")]
    pub p1: Option<f64>,
    #[serde_as(deserialize_as = "Option<DisplayFromStr>")]
    pub p2: Option<f64>,
    #[serde_as(deserialize_as = "Option<DisplayFromStr>")]
    pub p3: Option<f64>,
    #[serde_as(deserialize_as = "Option<DisplayFromStr>")]
    pub p4: Option<f64>,
    #[serde_as(deserialize_as = "Option<DisplayFromStr>")]
    pub p5: Option<f64>,
    #[serde_as(deserialize_as = "Option<DisplayFromStr>")]
    pub p6: Option<f64>,
    #[serde_as(deserialize_as = "Option<DisplayFromStr>")]
    pub p7: Option<f64>,
    #[serde_as(deserialize_as = "Option<DisplayFromStr>")]
    pub p8: Option<f64>,
    #[serde_as(deserialize_as = "Option<DisplayFromStr>")]
    pub p9: Option<f64>,

