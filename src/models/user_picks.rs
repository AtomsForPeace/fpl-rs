use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserPicks {
    pub active_chip: Value,
    pub automatic_subs: Vec<Value>,
    pub entry_history: EntryHistory,
    pub picks: Vec<Pick>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryHistory {
    pub event: i64,
    pub points: i64,
    pub total_points: i64,
    pub rank: i64,
    pub rank_sort: i64,
    pub overall_rank: i64,
    pub bank: i64,
    pub value: i64,
    pub event_transfers: i64,
    pub event_transfers_cost: i64,
    pub points_on_bench: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pick {
    pub element: i64,
    pub position: i64,
    pub multiplier: i64,
    pub is_captain: bool,
    pub is_vice_captain: bool,
}

