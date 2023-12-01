use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gameweek {
    pub elements: Vec<Element>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element {
    pub id: i64,
    pub stats: Stats,
    pub explain: Vec<Explain>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
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
    pub total_points: i64,
    pub in_dreamteam: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Explain {
    pub fixture: i64,
    pub stats: Vec<Stat>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stat {
    pub identifier: String,
    pub points: i64,
    pub value: i64,
}

