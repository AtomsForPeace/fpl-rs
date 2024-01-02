use serde::Deserialize;
use serde::Serialize;

pub type Transfers = Vec<Transfer>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transfer {
    pub element_in: i64,
    pub element_in_cost: i64,
    pub element_out: i64,
    pub element_out_cost: i64,
    pub entry: i64,
    pub event: i64,
    pub time: String,
}

