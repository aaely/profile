use leptos::*;
use log::log;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlStyleElement};
use crate::models::BarData;

#[component]
pub fn Resume() -> impl IntoView {

    view! {
        <div class="mygrid">
            <section class="header">
                <div class="title">
                <h3>"Aaron Ely          aaron.ely@hotmail.com"</h3>
                <p>{"Phone: (913) 547-2476"}</p>
                <a style="color: limegreen;" href="https://github.com/aaely">{"GitHub: github.com/aaely"}</a>
                </div>
                <img src="/static/images/profile.jpg" class="avatar"/>
            </section>
            <section class="main1">

            </section>
            <section class="main2">
                <div style="margin-bottom: 5%">
                <Languages />
                </div>
                <div style="margin-bottom: 5%">
                <Database />
                </div>
                <div style="margin-bottom: 5%">
                <AreaExpertise />
                </div>
            </section>

            <section class="main1">
                <h2>{"Work Experience"}</h2>
                <p><strong>{"IT Technician"}</strong></p>
                <p>{"Penn National Gaming 2017 - 2019"}</p>
                <ul>
                    <li>{"Troubleshot issues with client devices."}</li>
                    <li>{"Completed and helped with software deployment using SCCM"}</li>
                    <li>{"Troubleshot issues with MICROS terminals, server issues, and everything between."}</li>
                    <li>{"Programmed and configured Nortel IP phones"}</li>
                </ul>
                <br />
                <p><strong>{"System Test Engineer"}</strong></p>
                <p>{"Gaming Laboratories International 2019 - 2021"}</p>
                <ul>
                    <li>{"Tested back end casino systems for jurisdictional technical complaince"}</li>
                    <li>{"Developed test plans to ensure all requirements were met on a per jurisdiction basis"}</li>
                    <li>{"Configured slot machines, server configurations, and interface devices to test"}</li>
                    <li>{"Verified all reported mods either functioned as intended or did not"}</li>
                    <li>{"Wireshark network traffic to ensure encryption standards were met"}</li>
                    <li>{"Monitor SAS protocol messaging to ensure standards were being met"}</li>
                    <li>{"Created a React app to monitor the progress of submissions"}</li>
                    <ul>
                    <li>{"Front End: "}<a href="https://github.com/aaely/gli">{"https://github.com/aaely/gli"}</a></li>
                    </ul>
                </ul>
                <br />
                <p><strong>{"Inventory Clerk"}</strong></p>
                <p>{"Ryder Logistics 2024 - present"}</p>
                <ul>
                    <li>{"Investigate discrepancies in inventory quanties and resolve"}</li>
                    <li>{"Investigate discrepancies in expected reciepts and the actual physical material received"}</li>
                    <li>{"Automated receipt creation process and developed a scheduling application built entirely in Rust"}</li>
                    <ul>
                        <li>{"Front End: "}<a href="https://github.com/aaely/yew-app">{"https://github.com/aaely/yew-app"}</a></li>
                        <li>{"Back End: "}<a href="https://github.com/aaely/rocket_backend">{"https://github.com/aaely/rocket_backend"}</a></li>
                    </ul>
                </ul>
                <br />
                <h2>{"Projects"}</h2>
                <p><strong>{"Truck Scheduling"}</strong></p>
                <p>{"Front End: "}<a href="https://github.com/aaely/yew-app">{"https://github.com/aaely/yew-app"}</a></p>
                <p>{"Back End: "}<a href="https://github.com/aaely/rocket_backend">{"https://github.com/aaely/rocket_backend"}</a></p>
                <p>{"Description: A full stack web application built 100% in Rust. Automates the process of creating daily
                receipts at my current job. Implements WebSockets for live updates, and user authentication and role based 
                permissioning"}</p>
                <p><strong>{"Social Media Application"}</strong></p>
                <p>{"Front End: "}<a href="https://github.com/aaely/substrate-workshop-ui">{"https://github.com/aaely/substrate-workshop-ui"}</a></p>
                <p>{"Blockchain: "}<a href="https://github.com/aaely/parachain-workshop">{"https://github.com/aaely/parachain-workshop"}</a></p>
                <p>I built an ecommerce and social media platform using <a href="https://substrate.dev" target="_blank">Substrate</a> as
                the backend and database</p>
                <h2>{"Education"}</h2>
                <p><strong>{"B.Sc. Computer Science (in progress)"}</strong></p>
                <p>{"Southern New Hampshire University"}</p>
                <p>{"March 2025"}</p>
                <br />
            </section>
        </div>
    }
}

#[component]
fn Languages() -> impl IntoView {
    let data = create_rw_signal(vec![
        BarData { label: "Rust", value: 90 },
        BarData { label: "Javascript", value: 85 },
        BarData { label: "C++", value: 70 },
        BarData { label: "Golang", value: 60 },
        BarData { label: "Java", value: 50 },
    ]);

    if let Some(document) = window().unwrap().document() {
        let style = document.create_element("style").unwrap();
        style.set_attribute("type", "text/css").unwrap();
        document.head().unwrap().append_child(&style).unwrap();

        let style_sheet = style.dyn_into::<HtmlStyleElement>().unwrap();

        let mut keyframes = String::new();
        for (i, item) in data.get().iter().enumerate() {
            keyframes.push_str(&format!(
                "@keyframes language{} {{
                    from {{ width: 0%; }}
                    to {{ width: {}%; }}
                }}\n",
                i, item.value
            ));
            log::info!("{}", keyframes);
        }

        // Append the generated keyframes to the <style> element
        style_sheet.set_inner_html(&keyframes);
    }

    view! {
        <div class="chart-container">
            <h4 style="margin: 0 auto; margin-bottom: 3%;">Language Proficiency</h4>
            {move || {
                data.get().iter().enumerate().map(|(i, item)| {
                view! {
                    <div class="bar-container">
                        <div class="label">{item.label}</div>
                        <div class="bar" style={format!("animation: language{} 0.5s forwards;", i)}>{format!("{}%", item.value)}</div>
                    </div>
                }
                }).collect_view()}
            }
            </div>
    }
}

#[component]
fn Database() -> impl IntoView {
    let data = create_rw_signal(vec![
        BarData { label: "Neo4j", value: 90 },
        BarData { label: "SQL", value: 80 },
        BarData { label: "RocksDB", value: 70 },
    ]);

    if let Some(document) = window().unwrap().document() {
        let style = document.create_element("style").unwrap();
        style.set_attribute("type", "text/css").unwrap();
        document.head().unwrap().append_child(&style).unwrap();

        let style_sheet = style.dyn_into::<HtmlStyleElement>().unwrap();

        let mut keyframes = String::new();
        for (i, item) in data.get().iter().enumerate() {
            keyframes.push_str(&format!(
                "@keyframes database{} {{
                    from {{ width: 0%; }}
                    to {{ width: {}%; }}
                }}\n",
                i, item.value
            ));
        }

        // Append the generated keyframes to the <style> element
        style_sheet.set_inner_html(&keyframes);
    }

    view! {
        <div class="chart-container">
            <h4 style="margin: 0 auto; margin-bottom: 3%;">Database Proficiency</h4>
            {move || {
                data.get().iter().enumerate().map(|(i, item)| {
                view! {
                    <div class="bar-container">
                        <div class="label">{item.label}</div>
                        <div class="bar" style={format!("animation: database{} 0.5s forwards;", i)}>{format!("{}%", item.value)}</div>
                    </div>
                }
                }).collect_view()}
            }
            </div>
    }
}

#[component]
fn AreaExpertise() -> impl IntoView {
    let data = create_rw_signal(vec![
        BarData { label: "Front End", value: 90 },
        BarData { label: "Back End", value: 80 },
        BarData { label: "Database", value: 70 },
    ]);

    if let Some(document) = window().unwrap().document() {
        let style = document.create_element("style").unwrap();
        style.set_attribute("type", "text/css").unwrap();
        document.head().unwrap().append_child(&style).unwrap();

        let style_sheet = style.dyn_into::<HtmlStyleElement>().unwrap();

        let mut keyframes = String::new();
        for (i, item) in data.get().iter().enumerate() {
            keyframes.push_str(&format!(
                "@keyframes area{} {{
                    from {{ width: 0%; }}
                    to {{ width: {}%; }}
                }}\n",
                i, item.value
            ));
        }

        // Append the generated keyframes to the <style> element
        style_sheet.set_inner_html(&keyframes);
    }

    view! {
        <div class="chart-container">
            <h4 style="margin: 0 auto; margin-bottom: 3%;">Area Proficiency</h4>
            {move || {
                data.get().iter().enumerate().map(|(i, item)| {
                view! {
                    <div class="bar-container">
                        <div class="label">{item.label}</div>
                        <div class="bar" style={format!("animation: area{} 0.5s forwards;", i)}>{format!("{}%", item.value)}</div>
                    </div>
                }
                }).collect_view()}
            }
            </div>
    }
}