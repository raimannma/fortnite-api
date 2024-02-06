use crate::response_types::cosmetics::CosmeticsV2;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ShopV2 {
    pub hash: String,
    pub date: DateTime<Utc>,
    pub vbuck_icon: String,
    pub featured: ShopFeatured,
    // pub daily: Option<ShopDaily>,
    // pub votes: Option<ShopVotes>,
    // pub vote_winners: Option<ShopVoteWinners>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ShopFeatured {
    pub name: String,
    pub entries: Vec<ShopFeaturedEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ShopFeaturedEntry {
    pub regular_price: u64,
    pub final_price: u64,
    pub bundle: Option<ShopFeaturedEntryBundle>,
    pub banner: Option<ShopFeaturedEntryBanner>,
    pub giftable: bool,
    pub refundable: bool,
    pub sort_priority: i64,
    // pub categories: Option<_>,
    pub section_id: String,
    // pub section: Option<_>,
    pub layout: ShopFeaturedEntryLayout,
    pub dev_name: String,
    pub offer_id: String,
    pub display_asset_path: Option<String>,
    pub tile_size: String,
    pub new_display_asset_path: String,
    pub new_display_asset: ShopFeaturedEntryDisplayAsset,
    pub items: CosmeticsV2,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ShopFeaturedEntryBundle {
    pub name: String,
    pub info: String,
    pub image: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ShopFeaturedEntryBanner {
    pub value: String,
    pub intensity: String,
    pub backend_value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ShopFeaturedEntryLayout {
    pub id: String,
    pub name: String,
    pub category: Option<String>,
    pub index: i64,
    pub show_ineligible_offers: String,
    pub background: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ShopFeaturedEntryDisplayAsset {
    pub id: String,
    pub cosmetic_id: Option<String>,
    pub material_instances: Vec<ShopFeaturedEntryDisplayAssetMaterialInstance>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ShopFeaturedEntryDisplayAssetMaterialInstance {
    pub id: String,
    pub primary_mode: String,
    pub images: ShopFeaturedEntryDisplayAssetMaterialInstanceImages,
    pub colors: ShopFeaturedEntryDisplayAssetMaterialInstanceColors,
    pub scalings: HashMap<String, f64>,
    // pub flags: Option<_>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct ShopFeaturedEntryDisplayAssetMaterialInstanceImages {
    pub offer_image: String,
    pub background: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ShopFeaturedEntryDisplayAssetMaterialInstanceColors {
    #[serde(rename = "FallOff_Color")]
    pub fall_off_color: Option<String>,
    #[serde(rename = "Background_Color_A")]
    pub background_color_a: Option<String>,
    #[serde(rename = "Background_Color_B")]
    pub background_color_b: Option<String>,
}
