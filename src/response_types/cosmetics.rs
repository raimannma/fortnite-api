use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type CosmeticsV2 = Vec<CosmeticV2>;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CosmeticsNewV2 {
    pub build: String,
    pub previous_build: String,
    pub hash: String,
    pub date: DateTime<Utc>,
    pub last_addition: DateTime<Utc>,
    pub items: CosmeticsV2,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CosmeticV2 {
    pub id: String,
    pub name: String,
    pub description: String,
    pub r#type: CosmeticValue,
    pub rarity: CosmeticValue,
    pub series: Option<CosmeticSeries>,
    pub set: Option<CosmeticSet>,
    pub introduction: Option<CosmeticIntroduction>,
    pub images: CosmeticImages,
    pub variants: Option<Vec<CosmeticVariant>>,
    pub built_in_emote_ids: Option<Vec<String>>,
    pub search_tags: Option<Vec<CosmeticSearchTags>>,
    pub gameplay_tags: Option<Vec<String>>,
    pub meta_tags: Option<Vec<String>>,
    pub showcase_video: Option<String>,
    pub dynamic_pak_id: Option<String>,
    pub display_asset_path: Option<String>,
    pub definition_path: Option<String>,
    pub path: Option<String>,
    pub added: DateTime<Utc>,
    pub shop_history: Option<Vec<DateTime<Utc>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CosmeticValue {
    pub value: String,
    pub display_value: String,
    pub backend_value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CosmeticSet {
    pub value: Option<String>,
    pub text: Option<String>,
    pub backend_value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CosmeticImages {
    pub small_icon: Option<String>,
    pub icon: Option<String>,
    pub featured: Option<String>,
    // {"small":"https://fortnite-api.com/images/cosmetics/lego/character_agentsherbert/small.png","large":"https://fortnite-api.com/images/cosmetics/lego/character_agentsherbert/large.png","wide":null}
    pub lego: Option<CosmeticImagesLego>,
    pub other: Option<CosmeticImagesOther>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CosmeticImagesLego {
    pub small: String,
    pub large: String,
    pub wide: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CosmeticImagesOther {
    pub background: Option<String>,
    pub coverart: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CosmeticIntroduction {
    pub chapter: String,
    pub season: String,
    pub text: String,
    pub backend_value: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CosmeticSeries {
    pub value: String,
    pub image: Option<String>,
    pub colors: Vec<String>,
    pub backend_value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum CosmeticSearchTags {
    Bear,
    Food,
    Haze,
    Pink,
    Summer,
    Superman,
    Umbrella,
    Western,
    Winter,
    Yellow,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CosmeticVariant {
    pub channel: String,
    pub r#type: Option<String>,
    pub options: Vec<CosmeticVariantOption>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CosmeticVariantOption {
    pub tag: String,
    pub name: Option<String>,
    pub image: String,
}
