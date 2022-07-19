pub use crate::reddit::Client;

///
pub(crate) enum Subdomain {
    Oauth,
    Api,
}

///
impl Subdomain {
    pub(crate) fn domain_path(&self) -> &std::path::Path {
        match self {
            Subdomain::Api => std::path::Path::new("https://api.reddit.com"),
            Subdomain::Oauth => std::path::Path::new("https://oauth.reddit.com"),
        }
    }
}

///
pub(crate) enum Method {
    GET,
    POST,
    // PATCH,
}

///
pub(crate) trait Requester {
    ///
    fn method(&self) -> Method;

    ///
    fn subdomain(&self) -> Subdomain;

    ///
    fn path_url(&self) -> String;

    ///
    fn headers(&self, client: &std::sync::Arc<Client>) -> Option<reqwest::header::HeaderMap>;

    ///
    fn basic_auth(&self) -> Option<(String, Option<String>)>;

    ///
    fn body(&self) -> Option<String>;

    ///
    fn queries(&self) -> Option<std::collections::HashMap<String, String>>;
}
