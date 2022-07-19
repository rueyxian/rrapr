use serde::Deserialize;

///
// TODO incomplete fields
#[serde_with::serde_as]
#[derive(Deserialize, Debug)]
pub struct Comment {
    pub subreddit_id: String,

    pub author: String,
    pub author_fullname: String,
    pub author_patreon_flair: bool,
    pub author_premium: bool,
    pub author_flair_text: Option<String>,
    pub author_flair_css_class: Option<String>,

    #[serde(with = "crate::custom_serde::f64_serde_datetime")]
    pub created: chrono::DateTime<chrono::Utc>,
    #[serde(with = "crate::custom_serde::f64_serde_datetime")]
    pub created_utc: chrono::DateTime<chrono::Utc>,
    #[serde(with = "crate::custom_serde::false_or_f64_serde_opt_datetime")]
    pub edited: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(with = "crate::custom_serde::opt_f64_serde_opt_datetime")]
    pub banned_at_utc: Option<chrono::DateTime<chrono::Utc>>,
    pub banned_by: Option<String>,

    pub id: String,
    pub name: String,
    pub permalink: String,
    pub link_url: Option<String>,
    pub over_18: Option<bool>,
    pub body: Option<String>,
    pub body_html: Option<String>,

    pub score: f64,
    pub downs: u64,
    pub ups: u64,
    pub num_reports: Option<u64>,
    pub removal_reason: Option<String>,
    pub stickied: bool,
    pub locked: bool,
    pub archived: bool,
    //
    pub gilded: u64,
    pub gildings: serde_json::Value,
    //
    pub likes: Option<bool>,
    pub saved: bool,
    //
    // pub distinguished: Option<String>,
}
