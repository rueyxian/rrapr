mod setup;

///
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

///
#[tokio::test]
async fn auth() -> Result<()> {
    let client = setup::anon_client().await;
    assert!(client.username().is_none());
    println!("{:#?}", client);

    let client = setup::auth_client().await;
    assert!(client.username().is_some());
    println!("{:#?}", client);
    Ok(())
}

// ///
// #[tokio::test]
// async fn listing() -> Result<()> {
//     // let api_client = setup::get_client().await;
//
//     Ok(())
// }
