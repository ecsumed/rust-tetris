#[macro_use]
extern crate stdweb;

mod canvas;

use canvas::Canvas;

fn main() {
    let grid_color = "#CCCCCC";
    let block_color = "#5CB3FF";
    let grid_width = 10;
    let grid_height = 21;
    let grid_spacing = 20;

    stdweb::initialize();

    let canvas = Canvas::new("#canvas", grid_width, grid_height);

    canvas.draw_grid(grid_spacing, grid_color);

	stdweb::event_loop();
}
