use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MapV1 {
    pub images: MapImages,
    pub pois: Vec<MapPoi>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct MapImages {
    pub blank: String,
    pub pois: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MapPoi {
    pub id: String,
    pub name: Option<String>,
    pub location: MapPoiLocation,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MapPoiLocation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
