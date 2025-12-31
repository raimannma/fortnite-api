#[tokio::main]
async fn main() {
    let http_client = reqwest::Client::new();

    let result = fortnite_api::get_cosmetics_v2(&http_client, None).await;
    assert!(result.is_ok());

    let result = fortnite_api::get_cosmetics_new_v2(&http_client, None).await;
    assert!(result.is_ok());

    let cosmetic_id = result.unwrap().items.first().unwrap().id.clone();
    let result = fortnite_api::get_cosmetic_by_id_v2(&http_client, &cosmetic_id, None).await;
    println!("Result: {result:#?}");
    assert!(result.is_ok());
}
