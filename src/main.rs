use dioxus::prelude::*;
use glam::Vec2;

use crate::components::{CardComponent, FlipCard};

mod game;
mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");

// const HEADER_SVG: Asset = asset!("/assets/header.svg");

// altered version of KaTeX_Main to include filled "red" suits
const KATEX_SUITS: Asset = asset!("/assets/KaTeX_Suits.woff2");

// from https://www.confettijs.org/
const CONFETTI_JS: Asset = asset!("/assets/confetti.min.js");

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com",
        }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous",
        }
        document::Link {
            href: "https://fonts.googleapis.com/css2?family=Noto+Emoji:wght@300..700&family=Noto+Sans+Symbols+2&family=Noto+Sans+Symbols:wght@100..900&family=Noto+Sans:ital,wght@0,100..900;1,100..900&display=swap",
            rel: "stylesheet",
        }
        document::Link { rel: "icon", href: FAVICON }

        document::Style {
            // visibility hidden to prevent FOUC, is set back to visible in MAIN_CSS
            r#"
            html {{
                visibility: hidden;
            }}
            @font-face {{
                font-family: KaTeX_Suits;
                font-style: normal;
                font-weight: 700;
                src: url({KATEX_SUITS}) format("woff2");
            }} 
            "#,
        }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Script { src: CONFETTI_JS }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    let card = crate::game::Card::from_code("5W1").unwrap();

    rsx! {
        div {
            id: "hero",

            FlipCard {
                position: Vec2::new(10., 10.),
                card_width: 13.,
                card,
            }

            // img { src: HEADER_SVG, id: "header" }
            // div { id: "links",
            //     a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
            //     a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
            //     a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
            //     a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
            //     a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
            //     a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            // }
        }
    }
}
