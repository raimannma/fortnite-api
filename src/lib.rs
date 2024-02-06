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
use reqwest::Url;
use std::collections::HashMap;
use std::str::FromStr;

pub mod response_types;
pub mod utils;

pub async fn get_aes_keys_v2(
    http_client: &reqwest::Client,
    key_format: Option<AesKeyFormat>,
) -> reqwest::Result<AesV2> {
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
    let mut url = Url::from_str("https://fortnite-api.com/v1/banners").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut().append_pair("language", language);
    }

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_banners_colors_v1(
    http_client: &reqwest::Client,
) -> reqwest::Result<BannersColorsV1> {
    let url = Url::from_str("https://fortnite-api.com/v1/banners/colors").unwrap();

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_cosmetics_v2(
    http_client: &reqwest::Client,
    language: Option<&str>,
) -> reqwest::Result<CosmeticsV2> {
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
    let mut url = Url::from_str("https://fortnite-api.com/v2/cosmetics/br/new").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut().append_pair("language", language);
    }

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_cosmetics_by_id_v2(
    http_client: &reqwest::Client,
    cosmetic_id: &str,
    language: Option<&str>,
) -> reqwest::Result<CosmeticV2> {
    let mut url =
        Url::from_str(format!("https://fortnite-api.com/v2/cosmetics/br/{}", cosmetic_id).as_str())
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
    let mut url = Url::from_str("https://fortnite-api.com/v2/creatorcode").unwrap();
    url.query_pairs_mut().append_pair("name", name);

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_map_v1(
    http_client: &reqwest::Client,
    language: Option<&str>,
) -> reqwest::Result<MapV1> {
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
    let mut url = Url::from_str("https://fortnite-api.com/v1/playlists").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut().append_pair("language", language);
    }

    fetch_endpoint(http_client, url, "GET", "", &HashMap::new()).await
}

pub async fn get_playlists_by_id_v1(
    http_client: &reqwest::Client,
    playlist_id: &str,
    language: Option<&str>,
) -> reqwest::Result<PlaylistV1> {
    let mut url =
        Url::from_str(format!("https://fortnite-api.com/v1/playlists/{}", playlist_id).as_str())
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
    let mut url =
        Url::from_str(format!("https://fortnite-api.com/v2/stats/br/v2/{}", account_id).as_str())
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
