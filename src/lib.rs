#![deny(warnings)]
#![deny(unused)]
#![deny(clippy::expect_used)]
#![deny(clippy::get_unwrap)]
#![deny(clippy::exit)]
#![deny(clippy::indexing_slicing)]
#![deny(clippy::panic)]
#![deny(clippy::panic_in_result_fn)]
#![deny(clippy::unreachable)]
#![deny(clippy::print_stdout)]
#![allow(clippy::upper_case_acronyms)]

//! # Fortnite API
//! This crate is a wrapper for the Fortnite API.
//!
//! ## Usage
//!
//! ```rust no_run
//! #[tokio::main]
//! async fn main() {
//!     let http_client = reqwest::Client::new();
//!
//!     let result = fortnite_api::get_news_v2(&http_client, None).await;
//!     println!("Result: {result:#?}");
//!     assert!(result.is_ok());
//! }
//! ```
//!
//! ## Endpoints
//!
//! | Endpoint | Function | Result Type |
//! | --- | --- | --- |
//! | AES V2 | [`get_aes_keys_v2`] | [`AesV2`] |
//! | Banners V1 | [`get_banners_v1`] | [`BannersV1`] |
//! | Banners Colors V1 | [`get_banners_colors_v1`] | [`BannersColorsV1`] |
//! | Cosmetics V2 | [`get_cosmetics_v2`] | [`CosmeticsV2`] |
//! | Cosmetics New V2 | [`get_cosmetics_new_v2`] | [`CosmeticsNewV2`] |
//! | Cosmetic By ID V2 | [`get_cosmetic_by_id_v2`] | [`CosmeticV2`] |
//! | Creator Code V2 | [`get_creatorcode_v2`] | [`CreatorCodeV2`] |
//! | Map V1 | [`get_map_v1`] | [`MapV1`] |
//! | News V2 | [`get_news_v2`] | [`NewsV2`] |
//! | News Gamemode V2 | [`get_news_br_v2`] [`get_news_stw_v2`] [`get_news_creative_v2`] | [`News`] |
//! | Playlists V1 | [`get_playlists_v1`] | [`PlaylistsV1`] |
//! | Playlists By ID V1 | [`get_playlist_by_id_v1`] | [`PlaylistV1`] |
//! | Shop BR V2 | [`get_shop_br_v2`] [`get_shop_combined_v2`] | [`ShopV2`] |
//! | Stats V2 | [`get_stats_v2`] [`get_stats_by_account_id_v2`] | [`StatsV2`] |

use std::collections::HashMap;
use std::str::FromStr;

use reqwest::Url;

use crate::response_types::aes::{AesKeyFormat, AesV2};
use crate::response_types::banners::{BannersColorsV1, BannersV1};
use crate::response_types::cosmetics::{CosmeticV2, CosmeticsNewV2, CosmeticsV2};
use crate::response_types::creatorcode::CreatorCodeV2;
use crate::response_types::map::MapV1;
use crate::response_types::news::{News, NewsV2};
use crate::response_types::playlists::{PlaylistV1, PlaylistsV1};
use crate::response_types::shop::ShopV2;
use crate::response_types::stats::{StatsAccountType, StatsImage, StatsTimeWindow, StatsV2};
use crate::utils::fetch::fetch_endpoint;

pub mod response_types;
pub mod utils;

pub async fn get_aes_keys_v2(
    http_client: &reqwest::Client,
    key_format: Option<AesKeyFormat>,
) -> reqwest::Result<AesV2> {
    //! Get the current AES keys.
    //!
    //! ## Parameters
    //!
    //! - `http_client`: The reqwest client.
    //! - `key_format`: The format of the AES keys. Can be `None`, `Some(AesKeyFormat::Hex)`, or `Some(AesKeyFormat::Base64)`.
    //!
    //! ## Returns
    //!
    //! The current AES keys.
    //!
    //! ## Example
    //!
    //! ```rust
    //! use fortnite_api::response_types::aes::AesKeyFormat;
    //!
    //! #[tokio::main]
    //! async fn main() {
    //!     let http_client = reqwest::Client::new();
    //!
    //!     let result = fortnite_api::get_aes_keys_v2(&http_client, Some(AesKeyFormat::Hex)).await;
    //!     println!("Result: {result:#?}");
    //!     assert!(result.is_ok());
    //! }
    //! ```
    let mut url = Url::from_str("https://fortnite-api.com/v2/aes").unwrap();
    if let Some(key_format) = key_format {
        url.query_pairs_mut()
            .append_pair("keyFormat", &key_format.to_string());
    }

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_banners_v1(
    http_client: &reqwest::Client,
    language: Option<&str>,
) -> reqwest::Result<BannersV1> {
    //! Get the banners.
    //!
    //! ## Parameters
    //!
    //! - `http_client`: The reqwest client.
    //! - `language`: The language of the banners. Can be `None` or a language code.
    //!
    //! ## Returns
    //!
    //! The banners.
    //!
    //! ## Example
    //!
    //! ```rust
    //! #[tokio::main]
    //! async fn main() {
    //!     let http_client = reqwest::Client::new();
    //!
    //!     let result = fortnite_api::get_banners_v1(&http_client, None).await;
    //!     println!("Result: {result:#?}");
    //!     assert!(result.is_ok());
    //! }
    //! ```
    let mut url = Url::from_str("https://fortnite-api.com/v1/banners").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut().append_pair("language", language);
    }

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_banners_colors_v1(
    http_client: &reqwest::Client,
) -> reqwest::Result<BannersColorsV1> {
    //! Get the banner colors.
    //!
    //! ## Parameters
    //!
    //! - `http_client`: The reqwest client.
    //!
    //! ## Returns
    //!
    //! The banner colors.
    //!
    //! ## Example
    //!
    //! ```rust
    //! #[tokio::main]
    //! async fn main() {
    //!     let http_client = reqwest::Client::new();
    //!
    //!     let result = fortnite_api::get_banners_colors_v1(&http_client).await;
    //!     println!("Result: {result:#?}");
    //!     assert!(result.is_ok());
    //! }
    //! ```
    let url = Url::from_str("https://fortnite-api.com/v1/banners/colors").unwrap();

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_cosmetics_v2(
    http_client: &reqwest::Client,
    language: Option<&str>,
) -> reqwest::Result<CosmeticsV2> {
    //! Get the cosmetics.
    //!
    //! ## Parameters
    //!
    //! - `http_client`: The reqwest client.
    //! - `language`: The language of the cosmetics. Can be `None` or a language code.
    //!
    //! ## Returns
    //!
    //! The cosmetics.
    //!
    //! ## Example
    //!
    //! ```rust
    //! #[tokio::main]
    //! async fn main() {
    //!     let http_client = reqwest::Client::new();
    //!
    //!     let result = fortnite_api::get_cosmetics_v2(&http_client, None).await;
    //!     assert!(result.is_ok());
    //! }
    //! ```
    let mut url = Url::from_str("https://fortnite-api.com/v2/cosmetics/br").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut().append_pair("language", language);
    }

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_cosmetics_new_v2(
    http_client: &reqwest::Client,
    language: Option<&str>,
) -> reqwest::Result<CosmeticsNewV2> {
    //! Get the new cosmetics.
    //!
    //! ## Parameters
    //!
    //! - `http_client`: The reqwest client.
    //! - `language`: The language of the cosmetics. Can be `None` or a language code.
    //!
    //! ## Returns
    //!
    //! The new cosmetics.
    //!
    //! ## Example
    //!
    //! ```rust
    //! #[tokio::main]
    //! async fn main() {
    //!     let http_client = reqwest::Client::new();
    //!
    //!     let result = fortnite_api::get_cosmetics_new_v2(&http_client, None).await;
    //!     assert!(result.is_ok());
    //! }
    //! ```
    let mut url = Url::from_str("https://fortnite-api.com/v2/cosmetics/br/new").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut().append_pair("language", language);
    }

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_cosmetic_by_id_v2(
    http_client: &reqwest::Client,
    cosmetic_id: &str,
    language: Option<&str>,
) -> reqwest::Result<CosmeticV2> {
    //! Get the cosmetic by ID.
    //!
    //! ## Parameters
    //!
    //! - `http_client`: The reqwest client.
    //! - `cosmetic_id`: The ID of the cosmetic.
    //! - `language`: The language of the cosmetic. Can be `None` or a language code.
    //!
    //! ## Returns
    //!
    //! The cosmetic by ID.
    //!
    //! ## Example
    //!
    //! ```rust
    //! #[tokio::main]
    //! async fn main() {
    //!     let http_client = reqwest::Client::new();
    //!
    //!     let result = fortnite_api::get_cosmetics_new_v2(&http_client, None).await;
    //!     assert!(result.is_ok());
    //!
    //!     let cosmetic_id = result.unwrap().items.first().unwrap().id.clone();
    //!     let result = fortnite_api::get_cosmetic_by_id_v2(&http_client, &cosmetic_id, None).await;
    //!     println!("Result: {result:#?}");
    //!     assert!(result.is_ok());
    //! }
    //! ```
    let mut url =
        Url::from_str(format!("https://fortnite-api.com/v2/cosmetics/br/{cosmetic_id}").as_str())
            .unwrap();
    if let Some(language) = language {
        url.query_pairs_mut().append_pair("language", language);
    }

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_creatorcode_v2(
    http_client: &reqwest::Client,
    name: &str,
) -> reqwest::Result<CreatorCodeV2> {
    //! Get the creator code.
    //!
    //! ## Parameters
    //!
    //! - `http_client`: The reqwest client.
    //! - `name`: The name of the creator code.
    //!
    //! ## Returns
    //!
    //! The creator code.
    //!
    //! ## Example
    //!
    //! ```rust
    //! #[tokio::main]
    //! async fn main() {
    //!     let http_client = reqwest::Client::new();
    //!
    //!     let result = fortnite_api::get_creatorcode_v2(&http_client, "trymacs").await;
    //!     println!("Result: {result:#?}");
    //!     assert!(result.is_ok());
    //! }
    //! ```
    let mut url = Url::from_str("https://fortnite-api.com/v2/creatorcode").unwrap();
    url.query_pairs_mut().append_pair("name", name);

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_map_v1(
    http_client: &reqwest::Client,
    language: Option<&str>,
) -> reqwest::Result<MapV1> {
    //! Get the map.
    //!
    //! ## Parameters
    //!
    //! - `http_client`: The reqwest client.
    //! - `language`: The language of the map. Can be `None` or a language code.
    //!
    //! ## Returns
    //!
    //! The map.
    //!
    //! ## Example
    //!
    //! ```rust
    //! #[tokio::main]
    //! async fn main() {
    //!     let http_client = reqwest::Client::new();
    //!
    //!     let result = fortnite_api::get_map_v1(&http_client, None).await;
    //!     println!("Result: {result:#?}");
    //!     assert!(result.is_ok());
    //! }
    //! ```
    let mut url = Url::from_str("https://fortnite-api.com/v1/map").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut().append_pair("language", language);
    }

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_news_v2(
    http_client: &reqwest::Client,
    language: Option<&str>,
) -> reqwest::Result<NewsV2> {
    //! Get the news.
    //!
    //! ## Parameters
    //!
    //! - `http_client`: The reqwest client.
    //! - `language`: The language of the news. Can be `None` or a language code.
    //!
    //! ## Returns
    //!
    //! The news.
    //!
    //! ## Example
    //!
    //! ```rust
    //! #[tokio::main]
    //! async fn main() {
    //!     let http_client = reqwest::Client::new();
    //!
    //!     let result = fortnite_api::get_news_v2(&http_client, None).await;
    //!     println!("Result: {result:#?}");
    //! }
    //! ```
    let mut url = Url::from_str("https://fortnite-api.com/v2/news").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut().append_pair("language", language);
    }

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_news_br_v2(
    http_client: &reqwest::Client,
    language: Option<&str>,
) -> reqwest::Result<News> {
    //! Get the battle royale news.
    //!
    //! ## Parameters
    //!
    //! - `http_client`: The reqwest client.
    //! - `language`: The language of the news. Can be `None` or a language code.
    //!
    //! ## Returns
    //!
    //! The battle royale news.
    //!
    //! ## Example
    //!
    //! ```rust
    //! #[tokio::main]
    //! async fn main() {
    //!     let http_client = reqwest::Client::new();
    //!
    //!     let result = fortnite_api::get_news_br_v2(&http_client, None).await;
    //!     println!("Result: {result:#?}");
    //! }
    //! ```
    let mut url = Url::from_str("https://fortnite-api.com/v2/news/br").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut().append_pair("language", language);
    }

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_news_stw_v2(
    http_client: &reqwest::Client,
    language: Option<&str>,
) -> reqwest::Result<News> {
    //! Get the save the world news.
    //!
    //! ## Parameters
    //!
    //! - `http_client`: The reqwest client.
    //! - `language`: The language of the news. Can be `None` or a language code.
    //!
    //! ## Returns
    //!
    //! The save the world news.
    //!
    //! ## Example
    //!
    //! ```rust
    //! #[tokio::main]
    //! async fn main() {
    //!     let http_client = reqwest::Client::new();
    //!
    //!     let result = fortnite_api::get_news_stw_v2(&http_client, None).await;
    //!     println!("Result: {result:#?}");
    //! }
    //! ```
    let mut url = Url::from_str("https://fortnite-api.com/v2/news/stw").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut().append_pair("language", language);
    }

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_news_creative_v2(
    http_client: &reqwest::Client,
    language: Option<&str>,
) -> reqwest::Result<News> {
    //! Get the creative news.
    //!
    //! ## Parameters
    //!
    //! - `http_client`: The reqwest client.
    //! - `language`: The language of the news. Can be `None` or a language code.
    //!
    //! ## Returns
    //!
    //! The creative news.
    //!
    //! ## Example
    //!
    //! ```rust
    //! #[tokio::main]
    //! async fn main() {
    //!     let http_client = reqwest::Client::new();
    //!
    //!     let result = fortnite_api::get_news_creative_v2(&http_client, None).await;
    //!     println!("Result: {result:#?}");
    //! }
    //! ```
    let mut url = Url::from_str("https://fortnite-api.com/v2/news/creative").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut().append_pair("language", language);
    }

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_playlists_v1(
    http_client: &reqwest::Client,
    language: Option<&str>,
) -> reqwest::Result<PlaylistsV1> {
    //! Get the playlists.
    //!
    //! ## Parameters
    //!
    //! - `http_client`: The reqwest client.
    //! - `language`: The language of the playlists. Can be `None` or a language code.
    //!
    //! ## Returns
    //!
    //! The playlists.
    //!
    //! ## Example
    //!
    //! ```rust
    //! #[tokio::main]
    //! async fn main() {
    //!     let http_client = reqwest::Client::new();
    //!
    //!     let result = fortnite_api::get_playlists_v1(&http_client, None).await;
    //!     println!("Result: {result:#?}");
    //!     assert!(result.is_ok());
    //! }
    //! ```
    let mut url = Url::from_str("https://fortnite-api.com/v1/playlists").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut().append_pair("language", language);
    }

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_playlist_by_id_v1(
    http_client: &reqwest::Client,
    playlist_id: &str,
    language: Option<&str>,
) -> reqwest::Result<PlaylistV1> {
    //! Get the playlist by ID.
    //!
    //! ## Parameters
    //!
    //! - `http_client`: The reqwest client.
    //! - `language`: The language of the playlists. Can be `None` or a language code.
    //!
    //! ## Returns
    //!
    //! The playlist by ID.
    //!
    //! ## Example
    //!
    //! ```rust
    //! #[tokio::main]
    //! async fn main() {
    //!     let http_client = reqwest::Client::new();
    //!
    //!     let result = fortnite_api::get_playlists_v1(&http_client, None).await;
    //!     println!("Result: {result:#?}");
    //!     assert!(result.is_ok());
    //!
    //!     let playlist_id = result.unwrap().first().unwrap().id.clone();
    //!     let result = fortnite_api::get_playlist_by_id_v1(&http_client, &playlist_id, None).await;
    //!     println!("Result: {result:#?}");
    //! }
    //! ```
    let mut url =
        Url::from_str(format!("https://fortnite-api.com/v1/playlists/{playlist_id}").as_str())
            .unwrap();
    if let Some(language) = language {
        url.query_pairs_mut().append_pair("language", language);
    }

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_shop_br_v2(
    http_client: &reqwest::Client,
    language: Option<&str>,
) -> reqwest::Result<ShopV2> {
    //! Get the battle royale shop.
    //!
    //! ## Parameters
    //!
    //! - `http_client`: The reqwest client.
    //! - `language`: The language of the shop. Can be `None` or a language code.
    //!
    //! ## Returns
    //!
    //! The battle royale shop.
    //!
    //! ## Example
    //!
    //! ```rust
    //! #[tokio::main]
    //! async fn main() {
    //!     let http_client = reqwest::Client::new();
    //!
    //!     let result = fortnite_api::get_shop_br_v2(&http_client, None).await;
    //!     println!("Result: {result:#?}");
    //!     assert!(result.is_ok());
    //! }
    //! ```
    let mut url = Url::from_str("https://fortnite-api.com/v2/shop/br").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut().append_pair("language", language);
    }

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_shop_combined_v2(
    http_client: &reqwest::Client,
    language: Option<&str>,
) -> reqwest::Result<ShopV2> {
    //! Get the combined shop.
    //!
    //! ## Parameters
    //!
    //! - `http_client`: The reqwest client.
    //! - `language`: The language of the shop. Can be `None` or a language code.
    //!
    //! ## Returns
    //!
    //! The combined shop.
    //!
    //! ## Example
    //!
    //! ```rust
    //! #[tokio::main]
    //! async fn main() {
    //!     let http_client = reqwest::Client::new();
    //!
    //!     let result = fortnite_api::get_shop_combined_v2(&http_client, None).await;
    //!     println!("Result: {result:#?}");
    //!     assert!(result.is_ok());
    //! }
    //! ```
    let mut url = Url::from_str("https://fortnite-api.com/v2/shop/br/combined").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut().append_pair("language", language);
    }

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_stats_v2(
    http_client: &reqwest::Client,
    api_key: String,
    name: &str,
    account_type: Option<StatsAccountType>,
    time_window: Option<StatsTimeWindow>,
    image: Option<StatsImage>,
) -> reqwest::Result<StatsV2> {
    //! Get the player stats.
    //!
    //! ## Parameters
    //!
    //! - `http_client`: The reqwest client.
    //! - `api_key`: Your Fortnite API key.
    //! - `name`: The name of the player.
    //! - `account_type`: The account type of the player. Can be `None` or [`StatsAccountType`].
    //! - `time_window`: The time window of the stats. Can be `None` or [`StatsTimeWindow`].
    //! - `image`: The image of the stats. Can be `None` or [`StatsImage`].
    //! - `language`: The language of the shop. Can be `None` or a language code.
    //!
    //! ## Returns
    //!
    //! The player stats.
    //!
    //! ## Example
    //!
    //! ```rust,ignore
    //! use fortnite_api::response_types::stats::{StatsAccountType, StatsImage, StatsTimeWindow};
    //!
    //! #[tokio::main]
    //! async fn main() {
    //!     dotenv::dotenv().ok();
    //!     let http_client = reqwest::Client::new();
    //!     let api_key = std::env::var("FORTNITE_API_KEY")
    //!         .expect("Please set the FORTNITE_API_KEY environment variable");
    //!
    //!     let result =
    //!         fortnite_api::get_stats_v2(&http_client, api_key.clone(), "Test", None, None, None).await;
    //!     println!("Result: {result:#?}");
    //!     assert!(result.is_ok());
    //! }
    //! ```
    let mut url = Url::from_str("https://fortnite-api.com/v2/stats/br/v2").unwrap();
    url.query_pairs_mut().append_pair("name", name);
    if let Some(account_type) = account_type {
        url.query_pairs_mut()
            .append_pair("accountType", &account_type.to_string().to_lowercase());
    }
    if let Some(time_window) = time_window {
        url.query_pairs_mut()
            .append_pair("timeWindow", &time_window.to_string().to_lowercase());
    }
    if let Some(image) = image {
        url.query_pairs_mut()
            .append_pair("image", &image.to_string().to_lowercase());
    }
    let headers = [("Authorization".to_string(), api_key)]
        .into_iter()
        .collect();

    fetch_endpoint(http_client, url, "GET", "", &headers).await
}

pub async fn get_stats_by_account_id_v2(
    http_client: &reqwest::Client,
    api_key: String,
    account_id: &str,
    time_window: Option<StatsTimeWindow>,
    image: Option<StatsImage>,
) -> reqwest::Result<StatsV2> {
    //! Get the player stats by account ID.
    //!
    //! ## Parameters
    //!
    //! - `http_client`: The reqwest client.
    //! - `api_key`: Your Fortnite API key.
    //! - `account_id`: The account ID of the player.
    //! - `time_window`: The time window of the stats. Can be `None` or [`StatsTimeWindow`].
    //! - `image`: The image of the stats. Can be `None` or [`StatsImage`].
    //!
    //! ## Returns
    //!
    //! The player stats.
    //!
    //! ## Example
    //!
    //! ```rust,ignore
    //! use fortnite_api::response_types::stats::{StatsAccountType, StatsImage, StatsTimeWindow};
    //!
    //! #[tokio::main]
    //! async fn main() {
    //!     dotenv::dotenv().ok();
    //!     let http_client = reqwest::Client::new();
    //!     let api_key = std::env::var("FORTNITE_API_KEY")
    //!         .expect("Please set the FORTNITE_API_KEY environment variable");
    //!
    //!     let result = fortnite_api::get_stats_by_account_id_v2(
    //!         &http_client,
    //!         api_key.clone(),
    //!         "3f20d6f579db4e7ba71d80fc18576db2",
    //!         None,
    //!         None,
    //!     )
    //!     .await;
    //!     println!("Result: {result:#?}");
    //!     assert!(result.is_ok());
    //! }
    //! ```
    let mut url =
        Url::from_str(format!("https://fortnite-api.com/v2/stats/br/v2/{account_id}").as_str())
            .unwrap();
    if let Some(time_window) = time_window {
        url.query_pairs_mut()
            .append_pair("timeWindow", &time_window.to_string().to_lowercase());
    }
    if let Some(image) = image {
        url.query_pairs_mut()
            .append_pair("image", &image.to_string().to_lowercase());
    }
    let headers = [("Authorization".to_string(), api_key)]
        .into_iter()
        .collect();

    fetch_endpoint(http_client, url, "GET", "", &headers).await
}
