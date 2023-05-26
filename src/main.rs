use crate::universe::Universe;
use canvas::{CanvasController, CELL_SIZE};
use dioxus::prelude::*;
use std::time::Duration;

mod canvas;
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
    let universe = use_ref(cx, Universe::new);
    let is_playing = use_state(cx, || true);

    let universe_for_loop = universe.clone();
    use_future(cx, (is_playing,), move |(is_playing,)| async move {
        while *is_playing.current() {
            {
                // caution this scope is necessary to ensure that all borrowed resources are dropped before entering the asynchronous workflow
                let universe = universe_for_loop.clone();
                let mut universe = universe.write_silent();
                CanvasController::with("game-of-life-canvas", &mut *universe).tick();
            }
            async_std::task::sleep(Duration::from_millis(1)).await;
        }
    });

    cx.render(rsx! {
        div { style: "{CONTAINER_STYLE}",
            PlayButton { is_playing: is_playing }
            Canvas { id: "game-of-life-canvas", universe: universe }
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

#[inline_props]
#[allow(non_snake_case)]
fn Canvas<'a>(cx: Scope<'a>, id: &'a str, universe: &'a UseRef<Universe>) -> Element {
    let (height, width) = (universe.read().height(), universe.read().width());
    let canvas_height = ((CELL_SIZE + 1) * height + 1) as i64;
    let canvas_width = ((CELL_SIZE + 1) * width + 1) as i64;
    let id = id.to_owned();
    cx.render(rsx! { canvas { id: id, height: canvas_height, width: canvas_width } })
}
