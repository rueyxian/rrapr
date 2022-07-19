use crate::reddit::User;
use crate::request::AccessTokenReq;
use crate::request::Requester;

///
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

///
#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    user: parking_lot::Mutex<User>,
}

///
impl Client {
    ///
    pub async fn new(user_agent: &str) -> Result<std::sync::Arc<Client>> {
        let client = reqwest::Client::new();
        let client = Client {
            client,
            user: parking_lot::Mutex::new(User::new_anonymous(user_agent.to_string())),
        };
        Ok(std::sync::Arc::new(client))
    }

    ///
    pub fn clone(self: &std::sync::Arc<Client>) -> std::sync::Arc<Client> {
        std::sync::Arc::clone(self)
    }

    ///
    pub fn username(self: &std::sync::Arc<Client>) -> Option<String> {
        self.user.lock().username()
    }

    ///
    pub(crate) fn user_agent(self: &std::sync::Arc<Client>) -> String {
        self.user.lock().user_agent()
    }

    ///
    pub(crate) fn token_type(self: &std::sync::Arc<Client>) -> Result<String> {
        let token = self
            .user
            .lock()
            .access_token()
            .map(|token| token.token_type);
        if let Some(token) = token {
            Ok(token)
        } else {
            // Err("TODO")
            todo!()
        }
    }

    ///
    pub(crate) fn access_token(&self) -> Result<String> {
        let token = self
            .user
            .lock()
            .access_token()
            .map(|token| token.access_token);
        if let Some(token) = token {
            Ok(token)
        } else {
            // Err("TODO")
            todo!()
        }
    }

    ///
    fn build_url(self: &std::sync::Arc<Client>, requester: &Box<dyn Requester>) -> String {
        let subdomain = requester.subdomain();
        let domain = subdomain.domain_path();
        let url = domain.join(requester.path_url());
        url.to_string_lossy().to_string()
    }

    ///
    pub(crate) fn build_request(
        self: &std::sync::Arc<Client>,
        requester: Box<dyn Requester>,
    ) -> reqwest::RequestBuilder {
        use crate::request::requester::Method;

        let url = self.build_url(&requester);

        let req = match requester.method() {
            Method::GET => self.client.get(url),
            Method::POST => self.client.post(url),
        };

        let req = match requester.headers(self) {
            Some(h) => req.headers(h),
            None => req,
        };

        let req = match requester.basic_auth() {
            Some((client_id, client_secret)) => req.basic_auth(client_id, client_secret),
            None => req,
        };

        let req = match requester.body() {
            Some(body) => req.body(body),
            None => req,
        };

        let req = match requester.queries() {
            Some(q) => req.query(&q),
            None => req,
        };

        req
    }

    ///
    pub(crate) async fn fetch<T>(
        self: &std::sync::Arc<Client>,
        requester: Box<dyn Requester>,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let req = self.build_request(requester);
        let res = req.send().await?;
        if res.status().is_success() {
            let access_token = res.json::<T>().await?;
            Ok(access_token)
        } else {
            // TODO
            todo!()
        }
    }

    // ///
    // pub async fn login(
    //     self: std::sync::Arc<ApiClient>,
    //     requester: Box<AccessTokenReq>,
    // ) -> Result<std::sync::Arc<ApiClient>> {
    //     use crate::response::AccessToken;
    //
    //     let (client_id, client_secret, username, password, user_agent) = requester.all_fields();
    //     let access_token = self.fetch::<AccessToken>(requester).await?;
    //     *self.user.lock() =
    //         User::new_authenticated(client_id, client_secret, username, password, access_token);
    //     *self.user_agent.lock() = user_agent;
    //     Ok(self)
    // }

    ///
    pub async fn login(
        self: &std::sync::Arc<Client>,
        client_id: String,
        client_secret: String,
        username: String,
        password: String,
        user_agent: String,
    ) -> Result<()> {
        use crate::response::AccessToken;
        let requester = AccessTokenReq::new_box(
            &client_id,
            &client_secret,
            &username,
            &password,
            &user_agent,
        );
        let access_token = self.fetch::<AccessToken>(requester).await?;
        *self.user.lock() = User::new_authenticated(
            user_agent,
            client_id,
            client_secret,
            username,
            password,
            access_token,
        );
        Ok(())
    }
}

// ================================

// ///
// #[derive(Debug)]
// pub struct ApiClient {
//     client: reqwest::Client,
//     // user: std::sync::Arc<parking_lot::Mutex<Box<dyn User>>>,
//     user: User,
//     user_agent: String,
// }
// ///
// impl ApiClient {
//     ///
//     pub async fn new_arc(
//         user_agent: &str,
//     ) -> Result<std::sync::Arc<parking_lot::Mutex<ApiClient>>> {
//         let client = reqwest::Client::new();
//         let client = ApiClient {
//             client,
//             user: User::new_anonymous(),
//             user_agent: user_agent.to_string(),
//         };
//         Ok(std::sync::Arc::new(parking_lot::Mutex::new(client)))
//     }
//
//     ///
//     pub async fn new(user_agent: &str) -> Result<ApiClient> {
//         // use crate::api_client::AnonymousUser;
//         let client = reqwest::Client::new();
//         let client = ApiClient {
//             client,
//             user: User::new_anonymous(),
//             user_agent: user_agent.to_string(),
//         };
//         Ok(client)
//     }
//
//     ///
//     pub(crate) fn user_agent(&self) -> &str {
//         self.user_agent.as_str()
//     }
//
//     ///
//     pub(crate) fn token_type(&self) -> Result<String> {
//         let token = self.user.access_token().map(|token| token.token_type);
//         if let Some(token) = token {
//             Ok(token)
//         } else {
//             // Err("TODO")
//             todo!()
//         }
//     }
//
//     ///
//     pub(crate) fn access_token(&self) -> Result<String> {
//         let token = self.user.access_token().map(|token| token.access_token);
//         if let Some(token) = token {
//             Ok(token)
//         } else {
//             // Err("TODO")
//             todo!()
//         }
//     }
//
//     ///
//     fn build_url(&self, requester: &Box<dyn Requester>) -> String {
//         use crate::request::requester::SubDomain;
//
//         let domain = match requester.sub_domain() {
//             SubDomain::Api => std::path::Path::new("https://api.reddit.com"),
//             SubDomain::Oauth => std::path::Path::new("https://oauth.reddit.com"),
//         };
//         let url = domain.join(requester.path_url());
//         url.to_string_lossy().to_string()
//     }
//
//     ///
//     pub fn build_request(&self, requester: Box<dyn Requester>) -> reqwest::RequestBuilder {
//         use crate::request::requester::Method;
//
//         let url = self.build_url(&requester);
//
//         println!("url: {}", url);
//
//         let req = match requester.method() {
//             Method::GET => self.client.get(url),
//             Method::POST => self.client.post(url),
//         };
//
//         let req = match requester.headers(self) {
//             Some(h) => req.headers(h),
//             None => req,
//         };
//
//         let req = match requester.basic_auth() {
//             Some((client_id, client_secret)) => req.basic_auth(client_id, client_secret),
//             None => req,
//         };
//
//         let req = match requester.body() {
//             Some(body) => req.body(body),
//             None => req,
//         };
//
//         let req = match requester.queries() {
//             Some(q) => req.query(&q),
//             None => req,
//         };
//
//         req
//     }
//
//     ///
//     pub async fn login(mut self, requester: Box<AccessTokenReq>) -> Result<ApiClient> {
//         use crate::response::AccessToken;
//
//         let (client_id, client_secret, username, password, user_agent) = requester.all_fields();
//         let access_token = self.fetch::<AccessToken>(requester).await?;
//         self.user =
//             User::new_authenticated(client_id, client_secret, username, password, access_token);
//         self.user_agent = user_agent;
//         Ok(self)
//     }
//
//     ///
//     pub async fn fetch<T>(&self, requester: Box<dyn Requester>) -> Result<T>
//     where
//         T: serde::de::DeserializeOwned,
//     {
//         let req = self.build_request(requester);
//         let res = req.send().await?;
//
//         if res.status().is_success() {
//             let access_token = res.json::<T>().await?;
//             Ok(access_token)
//         } else {
//             // TODO
//             todo!()
//         }
//     }
// }
