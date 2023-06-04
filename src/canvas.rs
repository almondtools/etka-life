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

impl<'a> CanvasController<'a> {
    pub fn with(id: &str, universe: &'a mut Universe) -> CanvasController<'a> {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        let canvas = document
            .get_element_by_id(id)
            .expect("expecting a canvas in the document")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        Self { context, universe }
    }

    pub fn tick(&mut self) {
        self.universe.tick();

        self.draw_cells();
        self.draw_grid();
    }

    fn draw_cells(&self) {
        self.context.begin_path();

        for row in 0..self.universe.height() {
            for col in 0..self.universe.width() {
                if self.universe.cell_at(row, col).is_alive() {
                    self.context.set_fill_style(&ALIVE_COLOR.into());
                } else {
                    self.context.set_fill_style(&DEAD_COLOR.into());
                }

                self.context.fill_rect(
                    (col * (CELL_SIZE + 1) + 1) as f64,
                    (row * (CELL_SIZE + 1) + 1) as f64,
                    CELL_SIZE as f64,
                    CELL_SIZE as f64,
                );
            }
        }

        self.context.stroke();
    }

    fn draw_grid(&self) {
        self.context.begin_path();
        self.context.set_stroke_style(&GRID_COLOR.into());

        for i in 0..=self.universe.width() {
            self.context.move_to((i * (CELL_SIZE + 1) + 1) as f64, 0f64);
            self.context.line_to(
                (i * (CELL_SIZE + 1) + 1) as f64,
                ((CELL_SIZE + 1) * self.universe.height() + 1) as f64,
            );
        }

        for i in 0..=self.universe.height() {
            self.context.move_to(0f64, (i * (CELL_SIZE + 1) + 1) as f64);
            self.context.line_to(
                ((CELL_SIZE + 1) * self.universe.width() + 1) as f64,
                (i * (CELL_SIZE + 1) + 1) as f64,
            );
        }

        self.context.stroke();
    }
}
