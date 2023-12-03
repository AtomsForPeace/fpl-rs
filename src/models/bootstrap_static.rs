use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;


pub type Players = Vec<Player>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BootstrapStatic {
    pub events: Vec<Event>,
    pub game_settings: GameSettings,
    pub phases: Vec<Phase>,
    pub teams: Vec<Team>,
    pub total_players: i64,
    pub elements: Players,
    pub element_stats: Vec<PlayerStat>,
    pub element_types: Vec<PlayerType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: i64,
    pub name: String,
    pub deadline_time: String,
    pub average_entry_score: i64,
    pub finished: bool,
    pub data_checked: bool,
    pub highest_scoring_entry: Option<i64>,
    pub deadline_time_epoch: i64,
    pub deadline_time_game_offset: i64,
    pub highest_score: Option<i64>,
    pub is_previous: bool,
    pub is_current: bool,
    pub is_next: bool,
    pub cup_leagues_created: bool,
    pub h2h_ko_matches_created: bool,
    pub chip_plays: Vec<ChipPlay>,
    pub most_selected: Option<i64>,
    pub most_transferred_in: Option<i64>,
    pub top_element: Option<i64>,
    pub top_element_info: Option<TopPlayerInfo>,
    pub transfers_made: i64,
    pub most_captained: Option<i64>,
    pub most_vice_captained: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChipPlay {
    pub chip_name: String,
    pub num_played: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopPlayerInfo {
    pub id: i64,
    pub points: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameSettings {
    pub league_join_private_max: i64,
    pub league_join_public_max: i64,
    pub league_max_size_public_classic: i64,
    pub league_max_size_public_h2h: i64,
    pub league_max_size_private_h2h: i64,
    pub league_max_ko_rounds_private_h2h: i64,
    pub league_prefix_public: String,
    pub league_points_h2h_win: i64,
    pub league_points_h2h_lose: i64,
    pub league_points_h2h_draw: i64,
    pub league_ko_first_instead_of_random: bool,
    pub cup_start_event_id: Value,
    pub cup_stop_event_id: Value,
    pub cup_qualifying_method: Value,
    pub cup_type: Value,
    pub squad_squadplay: i64,
    pub squad_squadsize: i64,
    pub squad_team_limit: i64,
    pub squad_total_spend: i64,
    pub ui_currency_multiplier: i64,
    pub ui_use_special_shirts: bool,
    pub ui_special_shirt_exclusions: Vec<Value>,
    pub stats_form_days: i64,
    pub sys_vice_captain_enabled: bool,
    pub transfers_cap: i64,
    pub transfers_sell_on_fee: f64,
    pub league_h2h_tiebreak_stats: Vec<String>,
    pub timezone: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Phase {
    pub id: i64,
    pub name: String,
    pub start_event: i64,
    pub stop_event: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Team {
    pub code: i64,
    pub draw: i64,
    pub form: Value,
    pub id: i64,
    pub loss: i64,
    pub name: String,
    pub played: i64,
    pub points: i64,
    pub position: i64,
    pub short_name: String,
    pub strength: i64,
    pub team_division: Value,
    pub unavailable: bool,
    pub win: i64,
    pub strength_overall_home: i64,
    pub strength_overall_away: i64,
    pub strength_attack_home: i64,
    pub strength_attack_away: i64,
    pub strength_defence_home: i64,
    pub strength_defence_away: i64,
    pub pulse_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub chance_of_playing_next_round: Option<i64>,
    pub chance_of_playing_this_round: Option<i64>,
    pub code: i64,
    pub cost_change_event: i64,
    pub cost_change_event_fall: i64,
    pub cost_change_start: i64,
    pub cost_change_start_fall: i64,
    pub dreamteam_count: i64,
    pub element_type: i64,
    pub ep_next: String,
    pub ep_this: String,
    pub event_points: i64,
    pub first_name: String,
    pub form: String,
    pub id: i64,
    pub in_dreamteam: bool,
    pub news: String,
    pub news_added: Option<String>,
    pub now_cost: i64,
    pub photo: String,
    pub points_per_game: String,
    pub second_name: String,
    pub selected_by_percent: String,
    pub special: bool,
    pub squad_number: Value,
    pub status: String,
    pub team: i64,
    pub team_code: i64,
    pub total_points: i64,
    pub transfers_in: i64,
    pub transfers_in_event: i64,
    pub transfers_out: i64,
    pub transfers_out_event: i64,
    pub value_form: String,
    pub value_season: String,
    pub web_name: String,
    pub minutes: i64,
    pub goals_scored: i64,
    pub assists: i64,
    pub clean_sheets: i64,
    pub goals_conceded: i64,
    pub own_goals: i64,
    pub penalties_saved: i64,
    pub penalties_missed: i64,
    pub yellow_cards: i64,
    pub red_cards: i64,
    pub saves: i64,
    pub bonus: i64,
    pub bps: i64,
    pub influence: String,
    pub creativity: String,
    pub threat: String,
    pub ict_index: String,
    pub starts: i64,
    pub expected_goals: String,
    pub expected_assists: String,
    pub expected_goal_involvements: String,
    pub expected_goals_conceded: String,
    pub influence_rank: i64,
    pub influence_rank_type: i64,
    pub creativity_rank: i64,
    pub creativity_rank_type: i64,
    pub threat_rank: i64,
    pub threat_rank_type: i64,
    pub ict_index_rank: i64,
    pub ict_index_rank_type: i64,
    pub corners_and_indirect_freekicks_order: Option<i64>,
    pub corners_and_indirect_freekicks_text: String,
    pub direct_freekicks_order: Option<i64>,
    pub direct_freekicks_text: String,
    pub penalties_order: Option<i64>,
    pub penalties_text: String,
    pub expected_goals_per_90: f64,
    pub saves_per_90: f64,
    pub expected_assists_per_90: f64,
    pub expected_goal_involvements_per_90: f64,
    pub expected_goals_conceded_per_90: f64,
    pub goals_conceded_per_90: f64,
    pub now_cost_rank: i64,
    pub now_cost_rank_type: i64,
    pub form_rank: i64,
    pub form_rank_type: i64,
    pub points_per_game_rank: i64,
    pub points_per_game_rank_type: i64,
    pub selected_rank: i64,
    pub selected_rank_type: i64,
    pub starts_per_90: f64,
    pub clean_sheets_per_90: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerStat {
    pub label: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerType {
    pub id: i64,
    pub plural_name: String,
    pub plural_name_short: String,
    pub singular_name: String,
    pub singular_name_short: String,
    pub squad_select: i64,
    pub squad_min_play: i64,
    pub squad_max_play: i64,
    pub ui_shirt_specific: bool,
    pub sub_positions_locked: Vec<i64>,
    pub element_count: i64,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let full_name = self.first_name.to_owned() + " " + self.second_name.as_str();
        write!(f, "<id: {}, name: {}>", self.id, full_name)
    }
}
