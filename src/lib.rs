use chrono::{Datelike, Local, NaiveDate};
use leptos::prelude::*;
use leptos_router::components::{Router, Route, Routes, A};
use leptos_router::path;

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    leptos::mount::mount_to_body(App);
}

//
// Navigation
//
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
                <button class="btn-nav">"Buy me a coffee"</button>
            </ul>
        </nav>
        <div style="height: 90px"></div>
    }
}

//
// Hero
//
#[component]
fn Hero() -> impl IntoView {
    view! {
        //TODO: profile icon
        <h1 class="hero-headline">"Hi, I'm TrichterIH"</h1>
        <h1 class="hero-headline">"A Developer"</h1>
        <div class="button-panel">
            // ! All SVGs are 30x30
            <a href="https://github.com/TrichterIHII"><button type="button" class="icon-btn" style="background-color: #404040;"><span inner_html=include_str!("../public/res/icons/github.svg")/></button></a>
            <a><button type="button" class="icon-btn" style="background-color: #ee69ea;"><span inner_html=include_str!("../public/res/icons/itch-io.svg")/></button></a>
            <a><button type="button" class="icon-btn" style="background-color: #f33a3a;"><span inner_html=include_str!("../public/res/icons/youtube.svg")/></button></a>
            <a><button type="button" class="icon-btn" style="background-color: #2f71eb;"><span inner_html=include_str!("../public/res/icons/discord.svg")/></button></a>
        </div>
    }
}

#[component]
fn AboutMe() -> impl IntoView {
    let today:NaiveDate = Local::now().date_naive();
    let mut age:i32 = today.year() - 2010;
    if today.month() < 10 || (today.month() == 10 && today.day() < 21) {
        age -= 1;
    }

    view! {
        <h1 class="hero-headline" >"About Me"</h1>
        <h2>"Quick Facts"</h2>
        <p>
            {"I'm Titus Lars Meyer and "} {age} {"year-old software developer based in Hanau. "}
            {"I have a passion for building performance efficient and complex algorithms and applications "}
            {"and am particularly interested in frontend development, low level systems and math. \n"}
        </p>
        <h2>"How I Got Here"</h2>
        <p>
            {"My journey into software development started with: Minecraft. "}
            {"I wanted to code my own Mods with my own Blocks, Entities and mechanics "}
            {"so my first programming language obviously was Java. "}
            {"I quickly noticed, i like programming and programmed multiple hours every day. "}
            {"After I finished my first Minecraft-Mod I realized: Minecraft-Mods are one of the worst thing programmable, "}
            {"because it is like playing a chess-game but pre-moving every piece. "}
            {"I wanted to code my own game. So I watched a video for a Java-Framework: "}
            {"Java-Swing (yeah, I know what you're thinking about, why the f*ck swing, but I had no clue about frameworks at this point). "}
            {"After I programmed a few nice features (moving cam, custom world gen, block-interactions, etc.), "}
            {"I realized: My code was TRASH! It was slow, the code was really bad, and i mixed camelCase and snake_case. "}
            {"So: I decided to learn another programming language. "}
            {"I wanted to learn a fast language, which was not too much different to Java. "}
            {"After a little bit of research, i went with the wonderful language C++ (thank you, Bjarne). "}
            {"I learned really fast and I was thirsty to code a bigger project. "}
            {"I began to code the first version of "}<a href="/projects">"HopperLang"</a>{", my own programming language. "}
            {"At the beginning I kept it simple, so it was interpreted, "}
            {"but I quickly updated it to a compiler. In the 2nd version the HopperLang code (.hpl) was compiled to C++. "}
            {"But after getting better in C++, I found "}<a href="https://github.com/llvm/llvm-project">"LLVM"</a>{". "}
            {"But from this point I was unstoppable: I wanted to go lowlevel. "}
            {"I learned other languages like C, Rust and the wonderful world of arm ASM to understand how everything works. "}
            {"Especially as I learned Assembly, I had to deal with the "}<a href="neumann">"Von Neumann architecture"</a>{". "}
            {"All in all my life got painful, but i like it. "}
        </p>
        <h2>"What Drives Me"</h2>
        <p>
            {"What drives me is the urge to understand how things truly work, "}
            {"not just on the surface, but all the way down to the fundamentals. "}
            {"I believe code doesn't just have to work: if it leaks memory because you forgot to call free(), it's not finished - it's broken. "}
            {"I also like to understand things completely, for example a string. "}
            {"The most tutorials just say: 'It can store letters', but it goes much deeper. "}
            {"How does it work? Why is it existing? I wanted to find answers for all these questions. "}
            {"That kind of thinking pushed me from high-level languages all the way down to Assembly. "}
            {"I think, anybody who wants to learn programming should begin with low-level programming like C or ASM. "}
            {"I also have a deep love for complexity, especially where code and math intersect. "}
            {"That's probably why my solutions aren't always the most straightforward ones. "}
            {"But I try to find the balance: no bloated 10k-line files but also no cryptic 100-liners. "}
            {"I try to keep it just clean, efficient and still readable code. "}
        </p>
    } 
}

#[component]
fn Projects() -> impl IntoView {
    view! {
        <h1 class="hero-headline">"My Projects"</h1>
        // * HopperLang
        <div class="card">
            <h2>"HopperLang"</h2>
            <p>
                {"HopperLang is my own programming language written in C++ via LLVM. "}
                {"The goal is to make code short but still easy — sometimes you start writing "}
                {"and forget where you wanted to go. HopperLang is designed to be fast to write and fast to run. "}
                {"It also comes with the HDK (HopperDevelopmentKit), which contains useful functions like math_max() or math_min(). "}
            </p>
            <a href="https://github.com/TrichterIHII/HopperLang"><button>"GitHub"</button></a>
        </div>
        // * McToolbox
        <div class="card">
            <h2>"McToolbox"</h2>
            <p>
                {"McToolbox is one of my Minecraft mods, written in Java — but the main part is an external C++ application. "}
                {"The mod collects data from the game and sends it to the C++ app whenever a connection is established. "}
                {"The C++ application connects to the Minecraft client, processes the received data and displays it in a separate window. "}
                {"The main feature is calculating the positions of block entities from pie chart data alone, "}
                {"highlighting the possible radius in a minimap inside the application. "}
                {"The mod is fully legal: it only sends data that is already visible in the pie chart, and only while it's open. "}
            </p>
            <a href="https://github.com/TrichterIHII/McToolbox_Program"><button>"GitHub"</button></a>
        </div>
        // * Graph-Visualizer
        <div class="card">
            <h2>"Graph-Visualizer"</h2>
            <p>
                {"The Graph Visualizer was programmed in a single day as a bonus project for school — "}
                {"the task was to build something small with JavaScript. "}
                {"I chose this because I like math, but it was also painful since I really don't like JavaScript. "}
                {"It's not well styled and not particularly fast (thanks, JavaScript), but it does what it needs to do. "}
                {"To try it out, click the link below and enter a number of lines in the first field. "}
                {"The number of lines controls how smooth the graph looks and how fast it renders — I recommend 1,000 or 10,000. "}
                {"The second field is more interesting: here you can enter any function, for example x^2, which equals f(x) = x². "}
                {"Since I used the Cloudflare API to parse functions, things like sin(x), cos(x) and tan(x) are supported as well. "}
            </p>
            <a href="https://github.com/TrichterIHII/Graph-Visualizer"><button>"GitHub"</button></a>
            <a href="https://trichterihii.github.io/Graph-Visualizer/"><button>"Webside"</button></a>
        </div>
        // * HexN
        <div class="card">
            <h2>"HexN"</h2>
            <p>
                {"HexN is one of my first C++ applications, originally built for myself "}
                {"to practice and convert between binary, octal, decimal and hexadecimal. "}
                {"Before making it public, I added some output formatting to make it easier to follow. "}
                {"You simply choose two number systems and pick either learning or translation mode. "}
                {"It also supports ASCII conversion, using a built-in CSV containing the full ASCII table. "}
            </p>
            <a href="https://github.com/TrichterIHII/HexN"><button>"GitHub"</button></a>
        </div>
    }
}

#[component]
fn SideQuests() -> impl IntoView {
    view! {
        <h1 class="hero-headline">"Side Quests"</h1>
    }
}

#[component]
fn Contact() -> impl IntoView {
    view! {
        <h1 class="hero-headline">"Contact Me"</h1>
    }
}
