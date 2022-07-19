pub use crate::reddit::Client;
use crate::request::requester::Method;
use crate::request::requester::Requester;
use crate::request::requester::Subdomain;

pub(crate) struct AccessTokenReq {
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
    user_agent: String,
}

///
impl AccessTokenReq {
    ///
    pub(crate) fn new_box(
        client_id: &str,
        client_secret: &str,
        username: &str,
        password: &str,
        user_agent: &str,
    ) -> Box<AccessTokenReq> {
        let req = AccessTokenReq {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            user_agent: user_agent.to_string(),
        };
        Box::new(req)
    }
}

///
impl Requester for AccessTokenReq {
    ///
    fn method(&self) -> Method {
        Method::POST
    }

    ///
    fn subdomain(&self) -> Subdomain {
        Subdomain::Api
    }

    ///
    fn path_url(&self) -> String {
        "api/v1/access_token".to_string()
    }

    ///
    fn headers(&self, _: &std::sync::Arc<Client>) -> Option<reqwest::header::HeaderMap> {
        let mut map = reqwest::header::HeaderMap::new();
        map.insert(
            reqwest::header::USER_AGENT,
            self.user_agent.parse().unwrap(),
        );
        Some(map)
    }

    ///
    fn basic_auth(&self) -> Option<(String, Option<String>)> {
        Some((
            self.client_id.to_string(),
            Some(self.client_secret.to_string()),
        ))
    }

    ///
    fn body(&self) -> Option<String> {
        Some(format!(
            "grant_type=password&username={}&password={}",
            self.username, self.password,
        ))
    }

    ///
    fn queries(&self) -> Option<std::collections::HashMap<String, String>> {
        None
    }
}
