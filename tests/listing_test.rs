mod setup;

///
// type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

///
#[tokio::test]
async fn listing_new() {
    let client = setup::auth_client().await;

    let subreddit = "malaysians";

    let req = rrapr::request::NewReq::new_box(&client, subreddit).set_limit(1);
    let res = req.fetch().await.unwrap();

    for submission in res.data().children() {
        let submission = submission.data();
        let id = &submission.id;
        let title = &submission.title;

        let req = rrapr::request::CommentsReq::new_box(&client, subreddit, id);
        let comments = req.fetch().await.unwrap();

        println!("{}", id);
        println!("{}", title);
        println!("{:#?}", comments);

        // client.fet

        println!("");
        //
    }

    // println!("{:#?}", res);
}
