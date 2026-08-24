use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{CARD_BORDER_RADIUS_RATIO, CARD_HEIGHT_RATIO, CardComponent, CardFrame, FlipCard, Movement, rem}, game::{AnimationAct, AnimationKey, Board, BoardPos, Card, NUM_DEPOTS}};

#[component]
pub fn BoardComponent(
    position: Vec2,
    board: Board,
    #[props(default)]
    onclick: EventHandler<BoardPos>,
    #[props(default)]
    oncontextmenu: EventHandler<BoardPos>,
    #[props(default)]
    animation_key: AnimationKey,
    #[props(default)]
    is_won: bool,
) -> Element {
    let card_width = 13f32;
    let card_height = card_width * CARD_HEIGHT_RATIO;
    let spacer_x = 1.5f32;
    let start_y = 2f32;

    let pos_x = {
        let w = 5.;
        let left = 50. - (w * card_width + (w-1.) * spacer_x) / 2.;
        move |i: usize| {
            left + (card_width + spacer_x) * i as f32
        }
    };

    let column_card_offset = Vec2::new(0., card_height / 2.);

    let get_pos = |depot: usize, ord: usize| {
        Vec2::new(pos_x(depot), start_y) + column_card_offset * ord as f32
    };

    let selected_height = if let Some(BoardPos { depot_index, card_index }) = board.selected {
        let d = board.depots[depot_index].len() - card_index - 1;

        card_height + column_card_offset.y * d as f32
    } else {0.};

    let moving_card = |p1: Vec2, p2: Vec2, card: Card| rsx! {
        Movement {
            src_translate_vec: p1 - p2,
            CardComponent {
                position: p2,
                width: card_width,
                card: card,
            }
        }
    };

    let anims = board.animation_acts.iter().enumerate().map(|(i, act)| {
        match act {
            AnimationAct::Move { cards, pos1, pos2 } => {
                let mut pos1 = *pos1;
                let mut pos2 = *pos2;

                let nodes = cards.iter().map(move |card| {
                    let p1 = get_pos(pos1.depot_index, pos1.card_index);
                    let p2 = get_pos(pos2.depot_index, pos2.card_index);
                    let res = moving_card(p1, p2, *card);
                    pos1.card_index += 1;
                    pos2.card_index += 1;
                    res
                });

                rsx! {
                    Fragment {
                        key: "{animation_key},{i}", // needed to force remounts, so animations don't get "stale" and refuse to replay
                        {nodes}
                    }
                }
            },
            &AnimationAct::Flip { card, pos } => {
                let p = get_pos(pos.depot_index, pos.card_index);
                rsx! {
                    Fragment {
                        key: "{animation_key},{i}", // needed to force remounts, so animations don't get "stale" and refuse to replay
                        FlipCard {
                            position: p,
                            card_width,
                            card,
                        }
                    }
                }
            },
        }
    });

    rsx! {
        div {
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),

            for depot in 0..NUM_DEPOTS {
                CardFrame { 
                    position: get_pos(depot, 0),
                    width: card_width,
                    onclick: move |_| {
                        onclick.call(BoardPos::new(depot, !0))
                    },
                    oncontextmenu: move |ev: Event<MouseData>| {
                        ev.prevent_default();
                        oncontextmenu.call(BoardPos::new(depot, !0))
                    },
                }

                for i in 0..board.depots[depot].len() {
                    if board.selected == Some(BoardPos::new(depot, i)) {
                        div {
                            position: "absolute",
                            top: rem(get_pos(depot, i).y),
                            left: rem(get_pos(depot, i).x),
                            width: rem(card_width),
                            height: rem(selected_height),
                            background_color: "#ff0",
                            border_radius: rem(card_width * CARD_BORDER_RADIUS_RATIO),
                            class: "selected-halo",
                        }
                    }

                    
                    CardComponent { 
                        position: get_pos(depot, i),
                        width: card_width,
                        card: board.depots[depot][i],
                        // number_hint: if !is_face_up(depot) {i + 1},
                        onclick: move |_| {
                            onclick.call(BoardPos::new(depot, i))
                        },
                        oncontextmenu: move |ev: Event<MouseData>| {
                            ev.prevent_default();
                            oncontextmenu.call(BoardPos::new(depot, i))
                        },
                    }
                }
            }

            {anims}

            if is_won {
                div {
                    position: "absolute",
                    top: rem(25.),
                    left: rem(17.5),
                    width: rem(59.),
                    background_color: "#505",
                    padding: rem(3.),
                    color: "#fff",
                    font_size: rem(7.),
                    border_radius: rem(2.),
                    text_align: "center",
                    "YOU WIN!",
                }
            }
        }
    }
}