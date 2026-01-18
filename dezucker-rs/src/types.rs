use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Represents a value that can be either a string or a number.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum IdField {
    String(String),
    Number(i64),
}

/// A holder for tags, where values are optional strings.
pub type TagHolder = HashMap<String, Option<String>>;

/// Represents a structured fragment of a post for rendering.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PostFragment {
    pub text: String,
    pub timestamp: i64,
    pub media_uri: Option<String>,
    pub web_uri: Option<String>,
    #[serde(default)]
    pub is_photo: bool,
    #[serde(default)]
    pub is_unknown: bool,
    #[serde(default)]
    pub is_meaningful: bool,
    #[serde(rename = "_raw")]
    pub raw: Option<Value>,
}

/// The main container for post data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FormattedPost {
    pub tid: Option<String>,
    pub id: Option<IdField>,
    pub text: String,
    pub timestamp: Option<IdField>, // TS/Kotlin allow string | number for timestamp
    pub attachments_count: Option<i32>,
    pub meaningful_entries_count: i32,
    pub tags: Option<Vec<TagHolder>>,
    pub fragments: Option<Vec<PostFragment>>,
    pub media: Option<Vec<PostFragment>>,
    #[serde(rename = "_raw")]
    pub raw: Option<Value>,
}
