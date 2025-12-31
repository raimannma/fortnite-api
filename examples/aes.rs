use fortnite_api::response_types::aes::AesKeyFormat;

#[tokio::main]
async fn main() {
    let http_client = reqwest::Client::new();

    let result = fortnite_api::get_aes_keys_v2(&http_client, None).await;
    println!("Result: {result:#?}");
    assert!(result.is_ok());

    let result = fortnite_api::get_aes_keys_v2(&http_client, Some(AesKeyFormat::Hex)).await;
    println!("Result: {result:#?}");
    assert!(result.is_ok());

    let result = fortnite_api::get_aes_keys_v2(&http_client, Some(AesKeyFormat::Base64)).await;
    println!("Result: {result:#?}");
    assert!(result.is_ok());
}
