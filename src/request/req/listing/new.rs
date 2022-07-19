use crate::reddit::Client;
use crate::request::Method;
use crate::request::Requester;
use crate::request::Subdomain;

use crate::response::Listing;
use crate::response::Submission;
use crate::response::Thing;

///
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

///
pub struct NewReq {
    client: Option<std::sync::Arc<Client>>,
    subreddit: String,
    before: Option<String>,
    after: Option<String>,
    count: usize,
    limit: usize,
    // show: Option<bool>,
    // sr_detail: Option<bool>,
}

///
impl NewReq {
    ///
    pub fn new_box(client: &std::sync::Arc<Client>, subreddit: &str) -> Box<NewReq> {
        let req = NewReq {
            client: Some(client.clone()),
            subreddit: subreddit.to_string(),
            before: None,
            after: None,
            count: 0,
            limit: 25,
        };
        Box::new(req)
    }

    ///
    pub fn set_before(mut self: Box<Self>, before: &str) -> Box<NewReq> {
        assert!(self.after.is_none());
        self.before = Some(before.to_string());
        self
    }

    ///
    pub fn set_after(mut self: Box<Self>, after: &str) -> Box<NewReq> {
        assert!(self.before.is_none());
        self.after = Some(after.to_string());
        self
    }

    ///
    pub fn set_count(mut self: Box<Self>, count: usize) -> Box<NewReq> {
        self.count = count;
        self
    }

    ///
    pub fn set_limit(mut self: Box<Self>, limit: usize) -> Box<NewReq> {
        assert!(limit <= 100);
        self.limit = limit;
        self
    }

    ///
    pub async fn fetch(mut self: Box<Self>) -> Result<Thing<Listing<Thing<Submission>>>> {
        let api_client = self.client.take().unwrap();
        api_client
            .fetch::<Thing<Listing<Thing<Submission>>>>(self)
            .await
    }
}

///
impl Requester for NewReq {
    ///
    fn method(&self) -> Method {
        Method::GET
    }

    ///
    fn subdomain(&self) -> Subdomain {
        Subdomain::Oauth
    }

    ///
    fn path_url(&self) -> String {
        format!("r/{}/new", self.subreddit)
    }

    ///
    fn headers(&self, client: &std::sync::Arc<Client>) -> Option<reqwest::header::HeaderMap> {
        let mut map = reqwest::header::HeaderMap::new();
        map.insert(
            reqwest::header::USER_AGENT,
            client.user_agent().parse().unwrap(),
        );
        let authorization_val = format!(
            "{} {}",
            client.token_type().unwrap(),
            client.access_token().unwrap()
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
        // Some("".to_string())
        None
    }

    ///
    fn queries(&self) -> Option<std::collections::HashMap<String, String>> {
        let mut map = std::collections::HashMap::<String, String>::with_capacity(2);
        map.insert("count".to_string(), format!("{}", self.count));
        map.insert("limit".to_string(), format!("{}", self.limit));
        if let Some(before) = self.before.as_ref() {
            map.insert("before".to_string(), before.to_string());
        }
        if let Some(after) = self.after.as_ref() {
            map.insert("after".to_string(), after.to_string());
        }
        Some(map)
    }
}
