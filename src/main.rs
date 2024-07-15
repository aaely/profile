mod state;
mod router;
mod bio;
mod nav;
mod git_user;
mod resume;
mod models;
mod algos;
use router::Router;
use leptos::*;
use state::GlobalState;
use nav::Nav;

#[component]
fn App() -> impl IntoView {
    let global_state = create_rw_signal(GlobalState::new());

    provide_context(global_state);

    view! {
        <Nav />
        <div style="
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: space-evenly;
        height: 100vh;
        width: 100vw;">
            <Router />
        </div>
    }
}

fn main() {
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");

    // Mount the App component to the body of the web page
    mount_to_body(|| view! { <App /> });
}
