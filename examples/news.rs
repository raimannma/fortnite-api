#[tokio::main]
async fn main() {
    let http_client = reqwest::Client::new();

    let result = fortnite_api::get_news_v2(&http_client, None).await;
    println!("Result: {:#?}", result);

    let result = fortnite_api::get_news_br_v2(&http_client, None).await;
    println!("Result: {:#?}", result);

    let result = fortnite_api::get_news_stw_v2(&http_client, None).await;
    println!("Result: {:#?}", result);

    let result = fortnite_api::get_news_creative_v2(&http_client, None).await;
    println!("Result: {:#?}", result);
}
