use super::league::Leagues;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub joined_time: String,
    pub started_event: i64,
    pub favourite_team: i64,
    pub player_first_name: String,
    pub player_last_name: String,
    pub player_region_id: i64,
    pub player_region_name: String,
    pub player_region_iso_code_short: String,
    pub player_region_iso_code_long: String,
    pub summary_overall_points: i64,
    pub summary_overall_rank: i64,
    pub summary_event_points: i64,
    pub summary_event_rank: Option<i64>,
    pub current_event: i64,
    pub leagues: Leagues,
    pub name: String,
    pub name_change_blocked: bool,
    pub kit: Value,
    pub last_deadline_bank: i64,
    pub last_deadline_value: i64,
    pub last_deadline_total_transfers: i64,
}
