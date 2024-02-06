use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type PlaylistsV1 = Vec<PlaylistV1>;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistV1 {
    pub id: String,
    pub name: String,
    pub sub_name: Option<String>,
    pub description: Option<String>,
    pub game_type: Option<PlaylistGameType>,
    pub rating_type: Option<PlaylistRatingType>,
    pub min_players: i64,
    pub max_players: i64,
    pub max_teams: i64,
    pub max_team_size: i64,
    pub max_squads: i64,
    pub max_squad_size: i64,
    pub is_default: bool,
    pub is_tournament: bool,
    pub is_limited_time_mode: bool,
    pub is_large_team_game: bool,
    pub accumulate_to_profile_stats: bool,
    pub images: PlaylistImages,
    pub gameplay_tags: Vec<String>,
    pub path: String,
    pub added: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistImages {
    pub showcase: Option<String>,
    pub mission_icon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum PlaylistRatingType {
    #[serde(rename = "delmar-challenge")]
    DelmarChallenge,
    #[serde(rename = "delmar-competitive")]
    DelmarCompetitive,
    #[serde(rename = "fun")]
    Fun,
    #[serde(rename = "largeTeam")]
    LargeTeam,
    #[serde(rename = "nobuild")]
    NoBuild,
    #[serde(rename = "ranked-br")]
    RankedBR,
    #[serde(rename = "ranked-zb")]
    RankedZB,
    #[serde(rename = "respawn")]
    Respawn,
    #[serde(rename = "solo")]
    Solo,
    #[serde(rename = "team")]
    Team,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum PlaylistGameType {
    #[serde(rename = "EFortGameType::BR")]
    BR,
    #[serde(rename = "EFortGameType::BRArena")]
    BRArena,
    #[serde(rename = "EFortGameType::BRLTM")]
    BRLtm,
    #[serde(rename = "EFortGameType::Creative")]
    Creative,
    #[serde(rename = "EFortGameType::CreativeLTM")]
    CreativeLTM,
    #[serde(rename = "EFortGameType::DelMar")]
    DelMar,
    #[serde(rename = "EFortGameType::Festival")]
    Festival,
    #[serde(rename = "EFortGameType::Playground")]
    Playground,
    #[serde(rename = "EFortGameType::Social")]
    Social,
    #[serde(rename = "EFortGameType::VKEdit")]
    VKEdit,
    #[serde(rename = "EFortGameType::VKPlay")]
    VKPlay,
    #[serde(rename = "EFortGameType::ZeroBuild")]
    ZeroBuild,
}
