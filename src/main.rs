use crate::universe::Universe;
use dioxus::prelude::*;
use game_of_life::{GameOfLife, CELL_SIZE};
use std::cell::RefCell;
use std::time::Duration;
use wasm_bindgen::JsCast;

mod game_of_life;
mod universe;

const CONTAINER_STYLE: &str = r"
    display: flex;
    flex-direction: column;
    align-items: center;
";

fn main() {
    dioxus_web::launch(App);
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    let universe = Universe::new();
    let canvas_height = (CELL_SIZE + 1) * universe.height() + 1;
    let canvas_width = (CELL_SIZE + 1) * universe.width() + 1;
    let is_playing = use_state(cx, || true);

    let game_of_life = use_state(cx, || RefCell::new(None::<GameOfLife>));

    let game_of_life_init: UseState<RefCell<Option<GameOfLife>>> = game_of_life.clone();
    use_effect(cx, (), move |()| async move {
        let context = get_context_2d();
        *game_of_life_init.borrow_mut() = Some(GameOfLife::new(context, universe));
    });

    let game_of_life_play = game_of_life.clone();
    use_effect(cx, (is_playing,), move |(is_playing,)| async move {
        while *is_playing.current() {
            if let Some(game_of_life) = game_of_life_play.borrow_mut().as_mut() {
                game_of_life.tick();
            }
            async_std::task::sleep(Duration::from_millis(1)).await;
        }
    });

    cx.render(rsx! {
        div { style: "{CONTAINER_STYLE}",
            PlayButton { is_playing: is_playing }
            canvas { id: "game-of-life-canvas", height: canvas_height as i64, width: canvas_width as i64 }
        }
    })
}

#[inline_props]
#[allow(non_snake_case)]
fn PlayButton<'a>(cx: Scope<'a>, is_playing: &'a UseState<bool>) -> Element {
    cx.render(rsx! {
        button { onclick: move |_| {
                is_playing.modify(|value| !value);
            },
            if *is_playing.get() { "⏸" } else { "▶" }
        }
    })
}

fn get_context_2d() -> web_sys::CanvasRenderingContext2d {
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");
    let canvas = document
        .get_element_by_id("game-of-life-canvas")
        .expect("expecting a canvas in the document")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}
