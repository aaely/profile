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
                <br />
                <h3>Fourth Project</h3>
                <a href="https://github.com/aaely/election-madness" target="_blank">Ethereum Election Application</a>
                <p>Here I made an application that uses Ethereum as the database for an election application. It allows a user to register using an Ethereum address in order to cast a vote for a candidate.
                After casting a vote, it shows the user who they voted for and the totals for the current election.</p>
                <br />
                <h3>Fifth Project</h3>
                <a href="https://github.com/aaely/flureerecoil" target="_blank">Cannabis Dispensary and Spotify Wrapper</a>
                <p>In this project I used <a href="https://flur.ee" target="_blank">FlureeDB</a> to create a cannabis dispensary that tracks inventory, uses ethereum payments, has an ethereum wallet in browser,
                uses websockets to make all updates live across all connected users, hits spotify and can search artists, songs, albums, and so on. This project was intended to help me understand traditional
                token authentication, crypto user authentication and permissioning, and how to manage live connections in applications using websockets. This used a blockchain GraphDB as the database, nodejs as
                the websocket server, referenced the ethereum chain to hit tokens, used <a href="https://recoiljs.org" target="_blank">Recoil</a> to manage global state.</p>
                <br />
                <h3>Sixth Project</h3>
                <a href="https://github.com/aaely/substrate-workshop-ui" target="_blank">Peptide Store, Cannabis Dispensary, Social Media App Frontend</a>
                <a href="https://github.com/aaely/parachain-workshop" target="_blank">Blockchain Backend & Database</a>
                <p>In this project I expanded on the previous application and made a real custom blockchain to hold peptide inventory and sales, cannabis inventory and sales, and a social media app (Facebook 2008 basically).
                The peptide store was the project that initially got me interested in programming. Peptides were lifechanging for me and I always wanted to get into the manufacturing and distribution of them.
                The cannabis dispensary was more something I thought had a big monetary potential. This could facilitate electronic payment for cannabis products while it was still not allowed.
                The social media portion was more I wanted to take on a project that would push me to consider design principals and data structuring to push me to be a better engineer. Overall this is the
                project I am most proud of as it is quite remarkable.</p>
                <br />
                <h3>Seventh Project</h3>
                <a href="https://github.com/aaely/yew_app" target="_blank">Trailer Scheduling Yew Frontend</a>
                <a href="https://github.com/aaely/rocket_backend" target="_blank">Trailer Scheduling Rocket Backend</a>
                <p>This app was made to make my life easier at my current job. I used Neo4j as a database, <a href="https://rocket.rs" target="_blank">Rust Rocket</a> as my backend, and 
                <a href="https://yew.rs" target="_blank">Rust Yew</a> as the frontend. I use this to schedule trailers at my current job. I build the database using an In Transit Report provided to us by General Motors.
                I merge all the data in the report into the database and it builds all receipts on a trailer by trailer basis. So at the start of the day we can download a csv that contians all the receipt data for the day
                and push it into the Warehouse Management System server so all our receipts for the day are automatically done. This was previously a very time consuming task. I managed to make it 
                very quick and easy. We no longer have to look up each trailer requested and find out which plants the parts are for (this is performed automatically by code and the database). 
                This app features full user authentication and role permissioning. I have both HTTP and HTTPS versions of this running. It uses websockets for live updates.</p>
            </div>
        </div>
    }
}