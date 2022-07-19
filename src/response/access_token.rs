use serde::Deserialize;

///
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AccessToken {
    pub access_token: String,
    pub expires_in: u64,
    pub scope: String,
    pub token_type: String,
}
