use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Leagues {
    pub classic: Vec<Classic>,
    pub h2h: Vec<Value>,
    pub cup: Cup,
    pub cup_matches: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Classic {
    pub id: i64,
    pub name: String,
    pub short_name: Option<String>,
    pub created: String,
    pub closed: bool,
    pub rank: Value,
    pub max_entries: Value,
    pub league_type: String,
    pub scoring: String,
    pub admin_entry: Option<i64>,
    pub start_event: i64,
    pub entry_can_leave: bool,
    pub entry_can_admin: bool,
    pub entry_can_invite: bool,
    pub has_cup: bool,
    pub cup_league: Value,
    pub cup_qualified: Value,
    pub entry_rank: i64,
    pub entry_last_rank: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cup {
    pub matches: Vec<Value>,
    pub status: Status,
    pub cup_league: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub qualification_event: Value,
    pub qualification_numbers: Value,
    pub qualification_rank: Value,
    pub qualification_state: Value,
}
