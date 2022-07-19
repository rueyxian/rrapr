mod access_token;
mod comment;
mod submission;

pub(crate) use crate::response::access_token::AccessToken;

pub use crate::response::comment::Comment;
pub use crate::response::submission::Submission;

// ===============================================

use serde::Deserialize;

// outdated reference: https://github.com/reddit-archive/reddit/wiki/JSON
///
#[derive(Deserialize, Debug)]
pub struct Thing<T> {
    pub kind: String,
    pub data: T,
}

///
impl<T> Thing<T> {
    pub fn data(&self) -> &T {
        &self.data
    }
}

///
#[derive(Deserialize, Debug)]
pub struct Listing<T> {
    pub modhash: Option<String>,
    pub after: Option<String>,
    pub before: Option<String>,
    pub dist: Option<usize>,
    pub children: Vec<T>,
}

///
impl<T> Listing<T> {
    pub fn children(&self) -> &Vec<T> {
        &self.children
    }
}
