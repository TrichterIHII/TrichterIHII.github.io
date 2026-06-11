use leptos::prelude::*;
use leptos_router::components::{Router, Route, Routes, A};
use leptos_router::path;

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Nav/>
            <Routes fallback=|| "404">
                <Route path=path!("/") view=Hero/>
                <Route path=path!("/about-me") view=AboutMe/>
                <Route path=path!("/projects") view=Projects/>
                <Route path=path!("/side-quests") view=SideQuests/>
                <Route path=path!("/contact") view=Contact/>
            </Routes>
        </Router>
    }
}

//
// Navigation
//
#[component]
fn Nav() -> impl IntoView {
    view! {
        <nav>
            <a href="/" class="logo">"TrichterIH"</a>
            <ul>
                <li><A href="/about-me">"About Me"</A></li>
                <li><A href="/projects">"Projects"</A></li>
                <li><A href="/side-quests">"Sidequests"</A></li>
                <li><A href="/contact">"Contact Me"</A></li>
                <button type="button" class="btn-nav">"Buy me a coffee"</button>
            </ul>
        </nav>
    }
}

//
// Hero
//
#[component]
fn Hero() -> impl IntoView {
    view! {
        <section>
            //TODO: profile icon
            <h1 class="hero-headline" style="margin-top: 20px;">"Hi, I'm TrichterIH"</h1>
            <h1 class="hero-headline">"A Developer"</h1>
            <div class="button-panel">
                // ! All SVGs are 30x30
                <button type="button" class="icon-btn" style="background-color: #404040;"><span inner_html=include_str!("../public/res/icons/github.svg")/></button>
                <button type="button" class="icon-btn" style="background-color: #ee69ea;"><span inner_html=include_str!("../public/res/icons/itch-io.svg")/></button>
                <button type="button" class="icon-btn" style="background-color: #f33a3a;"><span inner_html=include_str!("../public/res/icons/youtube.svg")/></button>
                <button type="button" class="icon-btn" style="background-color: #2f71eb;"><span inner_html=include_str!("../public/res/icons/discord.svg")/></button>
            </div>
        </section>
    }
}

#[component]
fn Projects() -> impl IntoView {
    view! {

    }
}

#[component]
fn AboutMe() -> impl IntoView {
    view! {

    }
}

#[component]
fn SideQuests() -> impl IntoView {
    view! {

    }
}

#[component]
fn Contact() -> impl IntoView {
    view! {
        
    }
}