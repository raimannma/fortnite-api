#[tokio::main]
async fn main() {
    let http_client = reqwest::Client::new();

    let result = fortnite_api::get_banners_v1(&http_client, None).await;
    println!("Result: {result:#?}");
    assert!(result.is_ok());

    let result = fortnite_api::get_banners_colors_v1(&http_client).await;
    println!("Result: {result:#?}");
    assert!(result.is_ok());
}
