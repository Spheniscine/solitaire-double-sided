use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{CARD_HEIGHT_RATIO, CardComponent, rem}, game::{ANIMATION_DURATION, Card}};

#[component]
pub fn Movement(
    src_translate_vec: Vec2,
    children: Element,
) -> Element {
    rsx! {
        div {
            style: "--translateX: {rem(src_translate_vec.x)}; --translateY: {rem(src_translate_vec.y)}; 
            animation: {ANIMATION_DURATION.as_secs_f32()}s movement;",
            {children},
        }
    }
}

#[component]
pub fn FlipCard(
    position: Vec2,
    card_width: f32,
    card: Card,
) -> Element {
    rsx! {
        div {
            class: "flip-card",
            left: rem(position.x),
            top: rem(position.y),
            width: rem(card_width),
            height: rem(card_width * CARD_HEIGHT_RATIO),
            
            div {
                class: "flip-card-inner",
                animation: "{ANIMATION_DURATION.as_secs_f32()}s flip",
                div {
                    class: "flip-card-front",
                    CardComponent {
                        position: Vec2::ZERO,
                        width: card_width,
                        card,
                    }
                }
                div {
                    class: "flip-card-back",
                    CardComponent {
                        position: Vec2::ZERO,
                        width: card_width,
                        card: !card,
                    }
                }
            }
        }
    }
}