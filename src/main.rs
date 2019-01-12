#![allow(dead_code)]
extern crate wasm_bindgen;
extern crate rand;

mod piece;
mod canvas;

use piece::Piece;
use piece::PieceKind;
use canvas::Canvas;

fn main() {
    let mut frame = Canvas::new(10, 20);
    
    frame.tick();

    println!("{}", frame);
}

