#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate stdweb;

mod geometry;
mod piece;
mod canvas;
mod tetris;

use canvas::Canvas;
use tetris::Tetris;

fn main() {
    let grid_color = "#CCCCCC";
    let block_color = "#5CB3FF";
    let grid_width = 10;
    let grid_height = 21;
    let grid_spacing = 20;

    stdweb::initialize();

    let mut tetris = Tetris::new(grid_width, grid_height); 
    let canvas = Canvas::new("#canvas", grid_width, grid_height, grid_spacing);

    canvas.clear();
    canvas.draw_grid(grid_color);
    canvas.draw_block(1, 1, block_color);

    tetris.tick();
	tetris.draw(canvas, block_color);
    // stdweb::console!(log, format!("{}", tetris));

	stdweb::event_loop();
}
