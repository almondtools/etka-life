use crate::universe::Universe;
use dioxus::prelude::*;
use game_of_life::{GameOfLife, CELL_SIZE};
use std::time::Duration;
use wasm_bindgen::JsCast;

mod game_of_life;
mod universe;

fn main() {
    dioxus_web::launch(App);
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    let universe = Universe::new();
    let canvas_height = (CELL_SIZE + 1) * universe.height() + 1;
    let canvas_width = (CELL_SIZE + 1) * universe.width() + 1;

    use_effect(cx, (), move |()| async move {
        let context = get_context_2d();
        let mut game_of_life = GameOfLife::new(context, universe);
        loop {
            game_of_life.tick();
            async_std::task::sleep(Duration::from_millis(1)).await;
        }
    });

    cx.render(rsx! {canvas { id: "game-of-life-canvas", height: canvas_height as i64, width: canvas_width as i64 }})
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
