use serde::Deserialize;


///
// #[derive(Deserialize, Debug)]
// pub struct Gallery {
//     pub id: String,
//     pub media_id: String,
// }

///
#[serde_with::serde_as]
#[derive(Deserialize, Debug)]
pub struct Submission {
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
    pub thumbnail: String,
    pub title: String,
    pub selftext: String,
    pub selftext_html: Option<String>,
    pub url: Option<String>,
    pub over_18: bool,

    pub is_self: bool,
    pub score: f64,
    pub downs: u64,
    pub ups: u64,
    pub upvote_ratio: f64,
    pub num_comments: u64,
    pub num_reports: Option<u64>,
    pub removal_reason: Option<String>,
    pub stickied: bool,
    pub locked: bool,
    pub archived: bool,

    pub gilded: u64,
    pub gildings: serde_json::Value,

    pub hidden: bool,
    pub clicked: bool,
    pub likes: Option<bool>,
    pub saved: bool,

    pub distinguished: Option<String>,
    // pub is_created_from_ads_ui: bool,
    // pub is_gallery: bool,
    // pub gallery_data: serde_json::Value,
    // pub is_crosspostable: bool,
}
