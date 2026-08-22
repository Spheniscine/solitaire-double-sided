use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::rem, game::Card};

pub const CARD_HEIGHT_RATIO: f32 = 13. / 12.;
pub const CARD_BORDER_RADIUS_RATIO: f32 = 1.5 / 12.;

pub const KATEX_SUITS_FONT_STR: &str = "KaTeX_Suits";

fn get_color(is_white: bool) -> &'static str {
    if is_white {"#fff"} else {"#000"}
}

fn render_rank(card: Card) -> Element {
    rsx! {
        span {
            font_family: KATEX_SUITS_FONT_STR,

            "{card.rank}"
        }
    }
}

fn render_back_rank(card: Card) -> Element {
    rsx! {
        div {
            style: "place-items: center;",
            height: "1.2em",
            aspect_ratio: 1,
            border_radius: "50%",
            background_color: get_color(!card.is_white),
            color: get_color(card.is_white),
            display: "grid",
            font_size: "0.75em",
            font_family: KATEX_SUITS_FONT_STR,

            "{card.back_rank}"
        }
    }
}

#[component]
pub fn CardComponent(
    position: Vec2,
    width: f32,
    card: Card,

    #[props(default)]
    onclick: EventHandler<MouseEvent>,
    #[props(default)]
    oncontextmenu: EventHandler<MouseEvent>,
) -> Element {
    let pt = width / 12.;
    let pt = |x: f32| {
        rem(x * pt)
    };

    rsx! {
        div {
            style: "place-items: center;",
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),
            background_color: get_color(card.is_white),
            color: get_color(!card.is_white),
            width: pt(11.),
            height: pt(12.),
            border: "{pt(0.25)} solid",
            border_color: if card.is_white {"#000"} else {"#868"},
            border_radius: rem(width * CARD_BORDER_RADIUS_RATIO),
            display: "grid",
            grid_template_columns: "50% 50%",
            grid_template_rows: "50% 50%",
            font_size: pt(5.),
            text_align: "center",
            padding: pt(0.25),

            onclick, oncontextmenu,

            div { display: "flex", align_items: "center", pointer_events: "none", {render_rank(card)}},
            div { display: "flex", align_items: "center", pointer_events: "none", {render_back_rank(card)}},
            div { display: "flex", align_items: "center", pointer_events: "none", {render_back_rank(card)}},
            div { display: "flex", align_items: "center", pointer_events: "none", {render_rank(card)}},
        }
    }
}