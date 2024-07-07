use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlStyleElement};
use crate::models::BarData;

#[component]
pub fn Resume() -> impl IntoView {

    view! {
        <div class="mycontainer">
            <section style="margin-bottom: 20px;">
                <h2>{"Contact Information"}</h2>
                <p>{"Name: John Doe"}</p>
                <p>{"Email: john.doe@example.com"}</p>
                <p>{"Phone: (123) 456-7890"}</p>
                <p>{"LinkedIn: linkedin.com/in/johndoe"}</p>
                <p>{"GitHub: github.com/johndoe"}</p>
            </section>
            <section style="margin-bottom: 5%;">
                <Languages />
            </section>

            <section style="margin-bottom: 5%;">
                <Database />
            </section>

            <section style="margin-bottom: 5%;">
                <AreaExpertise />
            </section>

            <section style="margin-bottom: 20px;">
                <h2>{"Education"}</h2>
                <p><strong>{"B.Sc. in Computer Science"}</strong></p>
                <p>{"University of Example, 2015 - 2019"}</p>
            </section>

            <section style="margin-bottom: 20px;">
                <h2>{"Experience"}</h2>
                <p><strong>{"Software Engineer"}</strong></p>
                <p>{"Example Company, 2020 - Present"}</p>
                <ul>
                    <li>{"Developed and maintained web applications using Rust and Leptos"}</li>
                    <li>{"Collaborated with cross-functional teams to deliver high-quality software"}</li>
                    <li>{"Implemented REST APIs and integrated third-party services"}</li>
                </ul>
            </section>

            <section style="margin-bottom: 20px;">
                <h2>{"Skills"}</h2>
                <ul>
                    <li>{"Programming Languages: Rust, JavaScript, Python"}</li>
                    <li>{"Web Technologies: HTML, CSS, Leptos, React"}</li>
                    <li>{"Tools: Git, Docker, Kubernetes"}</li>
                    <li>{"Databases: PostgreSQL, MongoDB"}</li>
                </ul>
            </section>

            <section style="margin-bottom: 20px;">
                <h2>{"Projects"}</h2>
                <p><strong>{"Project 1: Personal Website"}</strong></p>
                <p>{"Description: A personal website built with Leptos showcasing my portfolio and blog."}</p>

                <p><strong>{"Project 2: Chat Application"}</strong></p>
                <p>{"Description: A real-time chat application developed using Rust and WebSockets."}</p>
            </section>
        </div>
    }
}

#[component]
fn Languages() -> impl IntoView {
    let data = create_rw_signal(vec![
        BarData { label: "Rust", value: 90 },
        BarData { label: "Javascript", value: 90 },
        BarData { label: "C++", value: 70 },
        BarData { label: "Golang", value: 50 },
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
                "@keyframes example{} {{
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
            <h4 style="margin: 0 auto; margin-bottom: 3%;">Language Proficiency</h4>
            {move || {
                data.get().iter().enumerate().map(|(i, item)| {
                view! {
                    <div class="bar-container">
                        <div class="label">{item.label}</div>
                        <div class="bar" style={format!("animation: example{} 0.5s forwards;", i)}>{format!("{}%", item.value)}</div>
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
                "@keyframes example{} {{
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
                        <div class="bar" style={format!("animation: example{} 0.5s forwards;", i)}>{format!("{}%", item.value)}</div>
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
                "@keyframes example{} {{
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
                        <div class="bar" style={format!("animation: example{} 0.5s forwards;", i)}>{format!("{}%", item.value)}</div>
                    </div>
                }
                }).collect_view()}
            }
            </div>
    }
}