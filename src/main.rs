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
use stdweb::web::{
	event::KeyDownEvent,
	event::ClickEvent,
	IEventTarget
};

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
                tetris.borrow().draw(&canvas.borrow(), "#5CB3FF");
                tetris.borrow_mut().tick();
                // stdweb::console!(log, format!("{}", tetris.borrow()));
            },
            time,
        )
    }

    let game_timer = Rc::new(RefCell::new(Some(game_loop(tetris, canvas.clone(), interval))));
	
    // fn is_paused<T> (game_timer: Option<T>) -> bool {
    //     match game_timer {
    //         Some(_) => true,
    //         None => false
    //     }
    // }

	// fn play() {
	// 	unimplemented!();
	// }
	// 
	// fn pause() {
	// 	unimplemented!();
	// }

	// let button = stdweb::web::document().query_selector( "#play-pause" ).unwrap().unwrap();

	// button.add_event_listener(
    //     let game_timer = game_timer.clone();
    //     move |_: ClickEvent| {
    //     if is_paused() {
    //         play();
    //     } else {
    //         pause();
    //     }
	// });


	stdweb::event_loop();
}
