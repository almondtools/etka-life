use wasm_bindgen::JsCast;

use crate::universe::Universe;

pub const CELL_SIZE: u32 = 5;

const GRID_COLOR: &str = "#CCCCCC";
const DEAD_COLOR: &str = "#FFFFFF";
const ALIVE_COLOR: &str = "#000000";

pub struct CanvasController<'a> {
    context: web_sys::CanvasRenderingContext2d,
    universe: &'a mut Universe,
}

// implCanvasController
