use crate::reddit::Client;
use crate::request::Requester;
use crate::response::Comment;
use crate::response::Listing;
use crate::response::Thing;

///
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

///
/// TODO incomplete fields
pub struct CommentsReq {
    client: Option<std::sync::Arc<Client>>,
    subreddit: String,
    article: String,
    // comment: Option<String>,
    // context: u8,
}

///
impl CommentsReq {
    pub fn new_box(
        client: &std::sync::Arc<Client>,
        subreddit: &str,
        article: &str,
    ) -> Box<CommentsReq> {
        let req = CommentsReq {
            client: Some(std::sync::Arc::clone(client)),
            subreddit: subreddit.to_string(),
            article: article.to_string(),
        };
        Box::new(req)
    }

    pub async fn fetch(mut self: Box<Self>) -> Result<Vec<Thing<Listing<Thing<Comment>>>>> {
        let api_client = self.client.take().unwrap();
        api_client
            .fetch::<Vec<Thing<Listing<Thing<Comment>>>>>(self)
            .await
    }
}

///
impl Requester for CommentsReq {
    ///
    fn method(&self) -> crate::request::Method {
        crate::request::Method::GET
    }

    ///
    fn subdomain(&self) -> crate::request::Subdomain {
        crate::request::Subdomain::Oauth
    }

    ///
    fn path_url(&self) -> String {
        format!("r/{}/comments/{}", self.subreddit, self.article)
    }

    ///
    fn headers(&self, api_client: &std::sync::Arc<Client>) -> Option<reqwest::header::HeaderMap> {
        let mut map = reqwest::header::HeaderMap::new();
        map.insert(
            reqwest::header::USER_AGENT,
            api_client.user_agent().parse().unwrap(),
        );
        let authorization_val = format!(
            "{} {}",
            api_client.token_type().unwrap(),
            api_client.access_token().unwrap()
        )
        .parse()
        .unwrap();
        map.insert(reqwest::header::AUTHORIZATION, authorization_val);
        Some(map)
    }

    ///
    fn basic_auth(&self) -> Option<(String, Option<String>)> {
        None
    }

    ///
    fn body(&self) -> Option<String> {
        None
    }

    ///
    fn queries(&self) -> Option<std::collections::HashMap<String, String>> {
        // todo!()
        None
    }
}

// article	    ID36 of a link
//
// comment	    (optional) ID36 of a comment
//
// context	    an integer between 0 and 8
//
// depth	    (optional) an integer
//
// limit	    (optional) an integer
//
// showedits	boolean value
//
// showmedia	boolean value
//
// showmore	boolean value
//
// showtitle	boolean value
//
// sort	    one of (confidence, top, new, controversial, old, random, qa, live)
//
// sr_detail	(optional) expand subreddits
//
// theme	    one of (default, dark)
//
// threaded	boolean value
//
// truncate	an integer between 0 and 50
