use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CreatorCodeV2 {
    pub code: String,
    pub account: CreatorCodeAccount,
    pub status: CreatorCodeStatus,
    pub verified: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CreatorCodeAccount {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum CreatorCodeStatus {
    Active,
    Inactive,
}
