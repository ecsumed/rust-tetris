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

use stdweb::traits::*;
use stdweb::web::{event::KeyDownEvent, IEventTarget};

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // let grid_color = "#CCCCCC";
    // let block_color = "#5CB3FF";
    let grid_width = 10;
    let grid_height = 21;
    let grid_spacing = 20;
    let interval = 500;

    let tetris = Rc::new(RefCell::new(Tetris::new(grid_width, grid_height)));
    let canvas = Rc::new(RefCell::new(Canvas::new("#canvas", grid_width, grid_height, grid_spacing)));

    stdweb::initialize();

    stdweb::web::document().add_event_listener({
        let tetris = tetris.clone();
        let canvas = canvas.clone();
        move |event: KeyDownEvent| {
            match event.key().as_ref() {
                "ArrowLeft" => {
                    tetris.borrow_mut().piece_left();
                    tetris.borrow().draw(&canvas.borrow(), "#5CB3FF");
                },
                "ArrowRight" => {
                    tetris.borrow_mut().piece_right();
                    tetris.borrow().draw(&canvas.borrow(), "#5CB3FF");
                },
                "ArrowDown" => {
                    tetris.borrow_mut().tick();
                    tetris.borrow().draw(&canvas.borrow(), "#5CB3FF");
                },
                "ArrowUp" => {
                    tetris.borrow_mut().piece_rotate_clockwise();
                    tetris.borrow().draw(&canvas.borrow(), "#5CB3FF");
                },
                _ => {}
            };
        }
	});

    canvas.borrow().draw_grid("#CCCCCC");

    fn game_loop(tetris: Rc<RefCell<Tetris>>, canvas: Rc<RefCell<Canvas>>, time: u32) {
        stdweb::web::set_timeout(
            move || {
                game_loop(tetris.clone(), canvas.clone(), time);
                tetris.borrow_mut().tick();
                tetris.borrow().draw(&canvas.borrow(), "#5CB3FF");
            },
            time,
        );
    }

    game_loop(tetris, canvas.clone(), interval);

	stdweb::event_loop();
}
