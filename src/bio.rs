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
            width: 20vw;
            height: 20vw;
            border-radius: 50%;
            object-fit: cover;
            border: 2px solid #ddd;" />
            </div>
            <h4>{"This is a profile page I put together in web assembly using "}<a href="https://leptos.dev" target="_blank">{"Leptos (Rust)"}</a>{" in order to introduce myself and demonstrate the skills I have acquired over the years."}</h4>
            <p>{"Here I will walk you through my learning of development over the years."}</p>
            <br />
            <div style="
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: space-evenly;
            height: 100vh;
            width: 70vw;">
                <h3>First Project Ever</h3>
                <br />
                <a href="https://github.com/aaely/MyProject" target="_blank">My First Project</a>
                <p>I got a Udemy course that walked me through setting up a Nodejs Express, React/Redux, and MongoDB blog app that came equipped with an emailer, payment processor with Stripe, 
                and Oauth with PassportJS. I didnt really understand everything that I did at the time. In order to more fully understand what I was doing, I began to add in functionality 
                I was interested in personally. I always wanted to create an online store to sell peptide supplements so I added a store page and created the models for the database. I struggled,
                but after a couple of days, I managed to get it functional.</p>
                <br />
                <h3>Second Project</h3>
                <a href="https://github.com/aaely/mrwiki" target="_blank">M Resort Wiki</a>
                <p>I was working at the M Resort in the IT department and during our migration from the older version of sharepoint (2014 I believe) we lost the wiki functionality. We would use the
                wiki to log information about the applications we supported:</p>
                <ul>
                    <li>Application configurations</li>
                    <li>Server information</li>
                    <li>Contacts for support</li>
                    <li>Troubleshooting guides</li>
                </ul>
                <p>So to alleviate this loss, I made an application that used <a href="https://strapi.io/" target="_blank">Strapi</a> and React to fill this void. I used graphql as the query mechanism.</p>
                <br />
                <h3>Third Project</h3>
                <a href="https://github.com/aaely/gli" target="_blank">GLI Testing Dashboard</a>
                <p>This application followed much the same model as did the Wiki above it. Strapi Backend, MongoDB as Database, React Frontend. This app used the React Draft WYSIWYG rich text editor to 
                store information about modifications to be tested. The purpose of this application was to keep track of submission progress for application submissions. I could store a submission that
                that would have an application, a version, modifications associated with this version, allow me to mark mods as <b>Audit</b> (more stringent testing), <b>JIRA</b> (something was wrong), 
                <b>Complete</b> (testing was done and uploaded), or <b>Scoped</b> (manufacturer requested to not be tested). It would give graph representation of the status of all the mods so management could see
                the progress of the submission at a glance. I could upload any testing performed for the mods to the individual mod and at the end of the submission I could download all testing for the 
                entire submission and upload it to the GLI database. On the applications page I put information on how to configure the applications to talk with their supporting systems so others could
                help with testing and have some reference to configure the systems accordingly.</p>
            </div>
        </div>
    }
}