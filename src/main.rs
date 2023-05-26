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
    cx.render(rsx! { h1 { "Hello world!" } })

    // AppBody
}
