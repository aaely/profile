use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct GitHubUser {
    pub login: String,
    pub avatar_url: String,
    pub html_url: String,
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BarData {
    pub label: &'static str,
    pub value: usize,
}