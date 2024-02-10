#[tokio::main]
async fn main() {
    let http_client = reqwest::Client::new();

    let result = fortnite_api::get_playlists_v1(&http_client, None).await;
    println!("Result: {:#?}", result);
    assert!(result.is_ok());

    let playlist_id = result.unwrap().first().unwrap().id.clone();
    let result = fortnite_api::get_playlist_by_id_v1(&http_client, &playlist_id, None).await;
    println!("Result: {:#?}", result);
}
