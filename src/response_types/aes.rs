use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct AesV2 {
    pub build: String,
    pub main_key: String,
    pub dynamic_keys: Vec<DynamicKey>,
    pub updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct DynamicKey {
    pub pak_filename: String,
    pub pak_guid: String,
    pub key: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum AesKeyFormat {
    Hex,
    Base64,
}

impl Display for AesKeyFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AesKeyFormat::Hex => write!(f, "hex"),
            AesKeyFormat::Base64 => write!(f, "base64"),
        }
    }
}
