use leptos::*;
use crate::models::GitHubUser;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct GlobalState {
    pub current_view: String,
    pub last_view: String,
    pub user: Option<GitHubUser>,
}

impl GlobalState {
    pub fn new() -> Self {
        GlobalState {
            current_view: "bio".to_string(),
            last_view: "bio".to_string(),
            user: None,
        }
    }
}