use leptos::*;

#[component]
pub fn Bio() -> impl IntoView {

    view! {
        <div style="
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: space-evenly;
        height: 100vh;
        width: 100vw;
        margin-top: 7vh">
            <h1>{"Aaron Ely"}</h1>
            <div style="margin: 5%;">
            <img src="static/images/profile.jpg" style="
            width: 30vw;
            height: 30vw;
            border-radius: 50%;
            object-fit: cover;
            border: 2px solid #ddd;" />
            </div>
            <p>{"This is a profile page I put together in web assembly using "}<a href="https://leptos.dev" target="_blank">{"Leptos (Rust)"}</a>{" in order to introduce myself and demonstrate the skills I have acquired over the years."}</p>
        </div>
    }
}