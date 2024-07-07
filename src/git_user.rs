use leptos:: *;
use reqwest::Client;
use crate::{models::GitHubUser, state::GlobalState};

#[component]
pub fn GitUser() -> impl IntoView {
    let app_state = expect_context::<RwSignal<GlobalState>>();
    let client = Client::new();
    let (user, set_user) = create_slice(
        app_state, 
        |state| state.user.clone(), 
        |state, u| state.user = u);
    let err = create_rw_signal(None::<String>);

    let get_user = {
        move |_| {
            let client = client.clone();
            spawn_local(async move {
                match client
                    .get("https://api.github.com/users/aaely")
                    .header("User-Agent", "request")
                    .send()
                    .await {
                        Ok(response) => {
                            if let Ok(user_info) = response.json::<GitHubUser>().await {
                                set_user(Some(user_info));
                            } else {
                                err.set(Some("failed to parse user info".to_string()));
                            }
                        },
                        Err(error) => {
                            err.set(Some(format!("{:?}", error)));
                        }
                    }
            });
        }
    };

    view! {
        <>
            <h1>
            {"Github Page"}
            </h1>
            { move ||
            {if let Some(user) = user.get() {
                view! {
                    <div style="text-align: center;">
                        <img style="
                        width: 10vw;
                        height: 10vw;
                        border-radius: 50%;
                        object-fit: cover;
                        border: 2px solid #ddd;
                        " 
                        src={user.avatar_url.clone()} 
                        alt="User avatar" 
                        />
                        <h2>{user.name.clone().unwrap_or(user.login.clone())}</h2>
                        <p>{user.bio.clone().unwrap_or("No bio available".to_string())}</p>
                        <p><strong>"Company: "</strong>{user.company.clone().unwrap_or("N/A".to_string())}</p>
                        <p><strong>"Location: "</strong>{user.location.clone().unwrap_or("N/A".to_string())}</p>
                        <a href={user.html_url.clone()} target="_blank">"View on GitHub"</a>
                        <p>Direct link to the repo for this site: <a href="https://github.com/aaely/profile" target="_blank">Repo</a></p>
                    </div>
                }
            } else if let Some(error) = err.get() {
                view! { <div style="color: red;">{error}</div> }
            } else {
                view! { 
                    <div style="text-align: center;">
                        <p>On this page we will use the reqwest crate to pull my user information from github</p>
                        <button on:click={get_user.clone()}>Get User Profile</button>
                    </div> 
                }
            }}
            }
        </>
    }
}