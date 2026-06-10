use leptos::prelude::*;
use leptos::mount::mount_to_body;

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Nav/>
        <Hero/>
    }
}

#[component]
fn Nav() -> impl IntoView {
    view! {
        <nav>
            <li><a href="/">"Home"</a></li>
            <li><a href="#projects">"Projekte"</a></li>
        </nav>
    }
}

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section>

        </section>
    }
}
