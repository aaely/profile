use leptos::*;
use crate::state::GlobalState;
use crate::{bio::Bio, git_user::GitUser, resume::Resume};


#[component]
pub fn Router() -> impl IntoView {
    let app_state = expect_context::<RwSignal<GlobalState>>();

    let (current_view, set_current_view) = create_slice(
        app_state, 
        |state| state.current_view.clone(), 
        |state, v| state.current_view = v
    );

    {move || {
        match current_view.get().as_str() {
            "bio" => view! { <Bio /> },
            "git_user" => view! { <GitUser /> },
            "resume" => view! { <Resume /> },
            _ => view! {<NotFound />},
        }
    }}
}

#[component]
fn NotFound() -> impl IntoView {

    view! {
        <h1 style="text-align: center;">
            {"Not Found"}
        </h1>
    }
}