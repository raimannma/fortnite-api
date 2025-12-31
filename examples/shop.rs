#[tokio::main]
async fn main() {
    let http_client = reqwest::Client::new();

    let result = fortnite_api::get_shop_br_v2(&http_client, None).await;
    println!("Result: {result:#?}");
    assert!(result.is_ok());

    let result = fortnite_api::get_shop_combined_v2(&http_client, None).await;
    println!("Result: {result:#?}");
    assert!(result.is_ok());
}
