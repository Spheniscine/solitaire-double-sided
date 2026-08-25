use dioxus::prelude::*;

use crate::{components::{Emoji, VIDEO_GAMEPLAY, rem, render_rank}, game::{Card, GameState, ScreenState}};

#[component]
fn Emph(children: Element) -> Element {
    rsx! {
        strong {
            color: "#ff0",
            {children}
        }
    }
}

#[component]
pub fn Help(mut game_state: Signal<GameState>) -> Element {

    let rank_text = |rank: u8| {
        rsx! {
            span {
                font_size: "1.1em",
                {render_rank(Card { rank, back_rank: 1, is_white: false })}
            }
        }
    };

    rsx! {
        div {
            font_size: rem(4.),
            class: "help",

            div {
                text_align: "left",

                p {
                    margin_top: "0",
                    "The deck is a special deck with 25 double-faced cards. Each card has two ",Emph{"faces"}, ", one white 
                    and one black. Each face has a ",Emph{"rank"}, " from ",{rank_text(1)},"~",{rank_text(5)},", denoted by the 
                    larger uncircled numeral. The smaller circled numeral shows the rank on the other face. There is one card 
                    for each possible combination of ranks for the two faces."
                }

                p {
                    "Cards stack by ", Emph {"descending rank"}, " and " Emph {"alternating color"},
                    ". Such stacks of any size can be moved as a unit. An empty column may be filled by any card or stack."
                }

                p {
                    Emph {"NOTE:"}, " To move cards, click to select a card or stack, then click the destination. ", Emph{"“Drag and drop” is not required."}
                }

                p {
                    "Select an exposed card, then press the ", Emoji { text: "↩️" } " button to ", Emph {"flip"}, 
                    " it over to the other face. You may also ", Emph {"right-click / long-press"}, " a card to do so."
                }

                p {
                    "To ",Emph{"win the game"},", sort all cards into 5 stacks of 5 cards each, all of descending rank and
                    alternating color."
                }
            }

            div {
                position: "absolute",
                bottom: rem(2.),
                width: "92rem",
                display: "flex",
                justify_content: "center",

                a {
                    href: VIDEO_GAMEPLAY,
                    target: "_blank",
                    text_decoration: "none",
                    margin_right: rem(4.),
                    div {
                        width: rem(30.),
                        position: "relative",
                        class: "game-button",
                        "Example video"
                    }
                }

                div {
                    width: rem(30.),
                    position: "relative",
                    class: "game-button",
                    onclick: move |_| game_state.write().screen_state = ScreenState::Game,
                    "Back to game"
                }
            }
        }
    }
}