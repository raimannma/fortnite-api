#[tokio::main]
async fn main() {
    let http_client = reqwest::Client::new();

    let result = fortnite_api::get_creatorcode_v2(&http_client, "trymacs").await;
    println!("Result: {:#?}", result);
    assert!(result.is_ok());
}
