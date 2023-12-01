use serde::Deserialize;
use serde::Serialize;

pub type Fixtures = Vec<Fixture>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fixture {
    pub code: i64,
    pub event: Option<i64>,
    pub finished: bool,
    pub finished_provisional: bool,
    pub id: i64,
    pub kickoff_time: Option<String>,
    pub minutes: i64,
    pub provisional_start_time: bool,
    pub started: Option<bool>,
    pub team_a: i64,
    pub team_a_score: Option<i64>,
    pub team_h: i64,
    pub team_h_score: Option<i64>,
    pub stats: Vec<Stat>,
    pub team_h_difficulty: i64,
    pub team_a_difficulty: i64,
    pub pulse_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stat {
    pub identifier: String,
    pub a: Vec<A>,
    pub h: Vec<H>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct A {
    pub value: i64,
    pub element: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct H {
    pub value: i64,
    pub element: i64,
}

