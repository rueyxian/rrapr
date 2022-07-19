use serde::Deserialize;

///
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

///
#[derive(Debug, Deserialize)]
struct ApiAuth {
    pub client_id: String,
    pub client_secret: String,
}

///
#[derive(Debug, Deserialize)]
struct UserAuth {
    pub username: String,
    pub password: String,
}

///
struct Config {
    api_auth_path: std::path::PathBuf,
    user_auth_path: std::path::PathBuf,
}

///
impl Config {
    ///
    fn new() -> Config {
        let home_path = {
            let home = std::env::var("HOME").unwrap();
            std::path::Path::new(&home).to_path_buf()
        };
        let api_auth_path = home_path.join(".auth/redx/api_auth.json");
        let user_auth_path = home_path.join(".auth/redx/user_auth.json");
        Config {
            api_auth_path,
            user_auth_path,
        }
    }

    ///
    pub fn read_auth_file(&self) -> Result<(String, String, String, String)> {
        let api_auth: ApiAuth = {
            let file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(self.api_auth_path.as_path())
                .unwrap();
            let mut reader = std::io::BufReader::new(file);
            serde_json::from_reader(&mut reader).unwrap()
        };

        let user_auth: UserAuth = {
            let file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(self.user_auth_path.as_path())
                .unwrap();
            let mut reader = std::io::BufReader::new(file);
            serde_json::from_reader(&mut reader).unwrap()
        };

        Ok((
            api_auth.client_id,
            api_auth.client_secret,
            user_auth.username,
            user_auth.password,
        ))
    }
}

///
fn user_agent(username: Option<&str>) -> String {
    let username = match username {
        Some(name) => format!("/u/{}", name),
        None => "anonymous".to_string(),
    };
    format!(
        "{}:{}:{} (by {})",
        std::env::consts::OS,
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        username,
    )
}

///
pub async fn anon_client() -> std::sync::Arc<rrapr::reddit::Client> {
    let user_agent = user_agent(None);
    let client = rrapr::reddit::Client::new(user_agent.as_str())
        .await
        .unwrap();
    client
}

///
pub async fn auth_client() -> std::sync::Arc<rrapr::reddit::Client> {
    let client = anon_client().await;
    let config = Config::new();
    let (client_id, client_secret, username, password) = config.read_auth_file().unwrap();
    let user_agent = user_agent(Some(username.as_str()));
    client
        .login(client_id, client_secret, username, password, user_agent)
        .await
        .unwrap();
    client
}

