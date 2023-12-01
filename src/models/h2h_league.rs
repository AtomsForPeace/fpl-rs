use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct H2HLeague {
    pub has_next: bool,
    pub page: i64,
    pub results: Vec<Result>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result {
    pub id: i64,
    pub entry_1_entry: i64,
    pub entry_1_name: String,
    pub entry_1_player_name: String,
    pub entry_1_points: i64,
    pub entry_1_win: i64,
    pub entry_1_draw: i64,
    pub entry_1_loss: i64,
    pub entry_1_total: i64,
    pub entry_2_entry: i64,
    pub entry_2_name: String,
    pub entry_2_player_name: String,
    pub entry_2_points: i64,
    pub entry_2_win: i64,
    pub entry_2_draw: i64,
    pub entry_2_loss: i64,
    pub entry_2_total: i64,
    pub is_knockout: bool,
    pub league: i64,
    pub winner: Value,
    pub seed_value: Value,
    pub event: i64,
    pub tiebreak: Value,
    pub is_bye: bool,
    pub knockout_name: String,
}

