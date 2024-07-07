use leptos::*;
use crate::state::GlobalState;

#[component]
pub fn Nav() -> impl IntoView {
    let state = expect_context::<RwSignal<GlobalState>>();
    let (current_view, set_current_view) = create_slice(
        state,
        |state| state.current_view.clone(),
        |state, v| state.current_view = v
    );
    view! {
        <>
            <div style="
            display: flex;
            position: fixed;
            justify-content: space-around;
            align-content: center;
            width: 100vw;
            height: 7vh;
            background-color: #333;
            color: limegreen;
            flex-wrap: wrap;
            ">
                <div on:click=move |_| set_current_view("git_user".to_string())>
                    <p>{"GitHub"}</p>
                </div>
                <div on:click=move |_| set_current_view("bio".to_string())>
                    <p>{"Bio"}</p>
                </div>
                <div on:click=move |_| set_current_view("resume".to_string())>
                    <p>{"Resume"}</p>
                </div>
            </div>
        </>
    }
}