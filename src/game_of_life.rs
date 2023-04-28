use crate::universe::{Cell, Universe};

pub const CELL_SIZE: u32 = 5;

const GRID_COLOR: &str = "#CCCCCC";
const DEAD_COLOR: &str = "#FFFFFF";
const ALIVE_COLOR: &str = "#000000";

pub struct GameOfLife {
    context: web_sys::CanvasRenderingContext2d,
    universe: Universe,
}

impl GameOfLife {
    pub fn new(context: web_sys::CanvasRenderingContext2d, universe: Universe) -> Self {
        Self { context, universe }
    }

    pub fn tick(&mut self) {
        self.universe.tick();

        self.draw_cells();
        self.draw_grid();
    }

    fn draw_cells(&self) {
        self.context.begin_path();

        self.draw_cells_with_style(|cell| cell == Cell::Alive, ALIVE_COLOR);
        self.draw_cells_with_style(|cell| cell == Cell::Dead, DEAD_COLOR);

        self.context.stroke();
    }

    fn draw_cells_with_style(&self, condition: impl Fn(Cell) -> bool, style: &str) {
        self.context.set_fill_style(&style.into());
        for row in 0..self.universe.height() {
            for col in 0..self.universe.width() {
                if !condition(self.universe[(row, col)]) {
                    continue;
                }

                self.context.fill_rect(
                    (col * (CELL_SIZE + 1) + 1) as f64,
                    (row * (CELL_SIZE + 1) + 1) as f64,
                    CELL_SIZE as f64,
                    CELL_SIZE as f64,
                );
            }
        }
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
