use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StatsAccountType {
    Epic,
    Psn,
    Xbl,
}
impl Display for StatsAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StatsTimeWindow {
    Season,
    Lifetime,
}
impl Display for StatsTimeWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StatsImage {
    All,
    KeyboardMouse,
    Gamepad,
    Touch,
    None,
}
impl Display for StatsImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StatsV2 {
    pub account: StatsAccount,
    pub battle_pass: StatsBattlePass,
    pub image: Option<String>,
    pub stats: StatsStats,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StatsAccount {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StatsBattlePass {
    pub level: u32,
    pub progress: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StatsStats {
    pub all: Option<StatsStatsInput>,
    pub keyboard_mouse: Option<StatsStatsInput>,
    pub gamepad: Option<StatsStatsInput>,
    pub touch: Option<StatsStatsInput>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StatsStatsInput {
    pub overall: StatsStatsInputOverall,
    pub solo: Option<StatsStatsInputMode>,
    pub duo: Option<StatsStatsInputMode>,
    pub trio: Option<StatsStatsInputMode>,
    pub squad: Option<StatsStatsInputMode>,
    pub ltm: Option<StatsStatsInputMode>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StatsStatsInputOverall {
    pub score: u64,
    pub score_per_min: f64,
    pub score_per_match: f64,
    pub wins: u32,
    pub top_3: u32,
    pub top_5: u32,
    pub top_6: u32,
    pub top_10: u32,
    pub top_12: u32,
    pub top_25: u32,
    pub kills: u32,
    pub kills_per_min: f64,
    pub kills_per_match: f64,
    pub deaths: u32,
    pub kd: f64,
    pub matches: u32,
    pub win_rate: f64,
    pub minutes_played: u64,
    pub players_outlived: u32,
    pub last_modified: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StatsStatsInputMode {
    pub score: u64,
    pub score_per_min: f64,
    pub score_per_match: f64,
    pub wins: u32,
    pub top_3: Option<u32>,
    pub top_5: Option<u32>,
    pub top_6: Option<u32>,
    pub top_10: Option<u32>,
    pub top_12: Option<u32>,
    pub top_25: Option<u32>,
    pub kills: u32,
    pub kills_per_min: f64,
    pub kills_per_match: f64,
    pub deaths: u32,
    pub kd: f64,
    pub matches: u32,
    pub win_rate: f64,
    pub minutes_played: u64,
    pub players_outlived: u32,
    pub last_modified: DateTime<Utc>,
}
