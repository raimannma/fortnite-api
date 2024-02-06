use reqwest::{RequestBuilder, Url};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct APIData<T> {
    pub status: i32,
    pub data: T,
    pub error: Option<String>,
}

pub(crate) async fn fetch_endpoint<D: DeserializeOwned>(
    http_client: &reqwest::Client,
    uri: Url,
    method: &str,
    body: &str,
    headers: &HashMap<String, String>,
) -> reqwest::Result<D> {
    let raw_result = build_request(uri, http_client, method, body, headers)?
        .send()
        .await?;
    raw_result.json::<APIData<_>>().await.map(|d| d.data)
}

fn build_request(
    uri: Url,
    http_client: &reqwest::Client,
    method: &str,
    body: &str,
    headers: &HashMap<String, String>,
) -> reqwest::Result<RequestBuilder> {
    let method = reqwest::Method::from_str(method).unwrap_or_default();
    Ok(http_client
        .request(method, uri)
        .body(body.to_string())
        .headers(
            headers
                .iter()
                .map(|(key, value)| {
                    (
                        reqwest::header::HeaderName::from_str(key),
                        reqwest::header::HeaderValue::from_str(value),
                    )
                })
                .filter_map(|(key, value)| match (key, value) {
                    (Ok(key), Ok(value)) => Some((key, value)),
                    _ => None,
                })
                .collect(),
        ))
}
