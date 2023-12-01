use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassicLeague {
    pub new_entries: NewEntries,
    pub last_updated_data: String,
    pub league: League,
    pub standings: Standings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewEntries {
    pub has_next: bool,
    pub page: i64,
    pub results: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct League {
    pub id: i64,
    pub name: String,
    pub created: String,
    pub closed: bool,
    pub max_entries: Value,
    pub league_type: String,
    pub scoring: String,
    pub admin_entry: i64,
    pub start_event: i64,
    pub code_privacy: String,
    pub has_cup: bool,
    pub cup_league: Value,
    pub rank: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Standings {
    pub has_next: bool,
    pub page: i64,
    pub results: Vec<Result>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result {
    pub id: i64,
    pub event_total: i64,
    pub player_name: String,
    pub rank: i64,
    pub last_rank: i64,
    pub rank_sort: i64,
    pub total: i64,
    pub entry: i64,
    pub entry_name: String,
}

