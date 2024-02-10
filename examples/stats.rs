use fortnite_api::response_types::stats::{StatsAccountType, StatsImage, StatsTimeWindow};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let http_client = reqwest::Client::new();
    let api_key = std::env::var("FORTNITE_API_KEY")
        .expect("Please set the FORTNITE_API_KEY environment variable");

    let result =
        fortnite_api::get_stats_v2(&http_client, api_key.clone(), "Test", None, None, None).await;
    println!("Result: {:#?}", result);
    assert!(result.is_ok());

    let result = fortnite_api::get_stats_v2(
        &http_client,
        api_key.clone(),
        "Test",
        Some(StatsAccountType::Epic),
        None,
        None,
    )
    .await;
    println!("Result: {:#?}", result);
    assert!(result.is_ok());

    let result = fortnite_api::get_stats_v2(
        &http_client,
        api_key.clone(),
        "Test",
        None,
        Some(StatsTimeWindow::Season),
        None,
    )
    .await;
    println!("Result: {:#?}", result);
    assert!(result.is_ok());

    let result = fortnite_api::get_stats_v2(
        &http_client,
        api_key.clone(),
        "Test",
        None,
        None,
        Some(StatsImage::All),
    )
    .await;
    println!("Result: {:#?}", result);
    assert!(result.is_ok());

    let result = fortnite_api::get_stats_by_account_id_v2(
        &http_client,
        api_key.clone(),
        "3f20d6f579db4e7ba71d80fc18576db2",
        None,
        None,
    )
    .await;
    println!("Result: {:#?}", result);
    assert!(result.is_ok());

    let result = fortnite_api::get_stats_by_account_id_v2(
        &http_client,
        api_key.clone(),
        "3f20d6f579db4e7ba71d80fc18576db2",
        None,
        None,
    )
    .await;
    println!("Result: {:#?}", result);
    assert!(result.is_ok());

    let result = fortnite_api::get_stats_by_account_id_v2(
        &http_client,
        api_key.clone(),
        "3f20d6f579db4e7ba71d80fc18576db2",
        None,
        None,
    )
    .await;
    println!("Result: {:#?}", result);
    assert!(result.is_ok());
}
