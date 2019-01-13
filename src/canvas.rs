use wasm_bindgen::prelude::*;

use utils;
use std::fmt;

use piece::Block;
use piece::Piece;
use piece::PieceKind;

/// Public struct, exported to JavaScript.
#[wasm_bindgen]
pub struct Canvas {
    width: i32,
    height: i32,
    cells: Vec<u8>,
    active_piece: Piece,
}

/// Public functions, exported to JavaScript.
#[wasm_bindgen]
impl Canvas {
    pub fn new(width: i32, height: i32) -> Canvas {
		// log!("width: {}, height: {}", width, height);

		utils::set_panic_hook();

        let cells = (0..width * height).map(|_| 0).collect();

        Canvas {
            width: width,
            height: height,
            cells: cells,
            active_piece: Piece::new(PieceKind::SShape),
        }
    }

    pub fn tick(&mut self) {
        self.piece_disintegrate();
        if self.wont_collide(&self.active_piece.pre_drop()) {
            self.active_piece.drop();
        } else {
            self.piece_add(Piece::new(PieceKind::SShape));
        }
            
        self.piece_integrate();
    }

    pub fn piece_left(&mut self) {
        self.piece_disintegrate();
        if self.wont_collide(&self.active_piece.pre_left()) {
            self.active_piece.left();
        }
        self.piece_integrate();
    }
    
    pub fn piece_right(&mut self) {
        self.piece_disintegrate();
        if self.wont_collide(&self.active_piece.pre_right()) {
            self.active_piece.right();
        }
        self.piece_integrate();
    }

    pub fn piece_rotate_clockwise(&mut self) {
        self.piece_disintegrate();
        if self.wont_collide(&self.active_piece.pre_rotate_right()) {
            self.active_piece.rotate_right();
        }
        self.piece_integrate();
    }
    
    pub fn cells(&self) -> *const u8 {
        self.cells.as_ptr()
    }
}

/// Private functions.
impl Canvas {
    fn piece_integrate(&mut self) {
        for block in self.active_piece.blocks.iter() {
            let index = ((self.width * (block.y)) + (block.x)) as usize;
            self.cells[index] = 1;
        }
    }
    
    fn piece_disintegrate(&mut self) {
        for block in self.active_piece.blocks.iter() {
            let index = ((self.width * (block.y)) + (block.x)) as usize;
            self.cells[index] = 0;
        }
    }
    
    fn piece_add(&mut self, piece: Piece) {
        self.active_piece = piece;
    }

    fn wont_collide(&self, block: &Vec<Block>) -> bool {
        block.
        iter().
        all(
            |block| {
                block.x >= 0 &&
                block.x < (self.width) &&
                block.y >= 0 &&
                block.y < (self.height)
            }
        )
    }
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == 0 { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
