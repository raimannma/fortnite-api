use serde::{Deserialize, Serialize};

pub type BannersV1 = Vec<BannerV1>;
pub type BannersColorsV1 = Vec<BannersColorV1>;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct BannerV1 {
    pub id: String,
    pub dev_name: String,
    pub description: String,
    pub category: BannerCategory,
    pub full_usage_rights: bool,
    pub images: BannerImages,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct BannerImages {
    pub small_icon: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum BannerCategory {
    BattleRoyale,
    Founder,
    Other,
    Special,
    Standard,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct BannersColorV1 {
    pub id: String,
    pub color: String,
    pub category: String,
    pub sub_category_group: u8,
}
