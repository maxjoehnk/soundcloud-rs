use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum WebProfileKind {
    #[serde(rename = "web-profile")]
    WebProfile,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebProfile {
    pub kind: WebProfileKind,
    pub id: usize,
    pub service: String,
    pub title: String,
    pub url: String,
    pub username: Option<String>,
    pub created_at: String,
}
