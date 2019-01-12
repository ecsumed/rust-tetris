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
    active_piece: Option<Piece>,
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
            active_piece: Some(Piece::new(PieceKind::TShape).unwrap()),
        }
    }

    pub fn tick(&mut self) {
        self.piece_disintegrate();
   
        let drop_piece = self.wont_collide(&self.active_piece.as_ref().unwrap().pre_drop());  
        if let Some(piece) = &mut self.active_piece {
            //geometry::rotate_l(piece);
            
            if drop_piece {
                piece.drop();
            }
            
            //piece.right();
            //piece.right();
            //piece.right();
            //let floor_blocks = piece.get_floor_blocks();
            //println!("{:?}", floor_blocks);
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
        if let Some(piece) = &self.active_piece {
            for block in piece.blocks.iter() {
                let index = ((self.width * (block.y)) + (block.x)) as usize;
                self.cells[index] = 1;
            }
        }
    }
    
    fn piece_disintegrate(&mut self) {
        if let Some(piece) = &self.active_piece {
            for block in piece.blocks.iter() {
                let index = ((self.width * (block.y)) + (block.x)) as usize;
                self.cells[index] = 0;
            }
        }
    }
    

    pub fn piece_add(&mut self, piece: Piece) {
        self.active_piece = Some(piece);
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
