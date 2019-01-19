use std::fmt;

use piece::Block;
use piece::Piece;
use piece::PieceKind;
use canvas::Canvas;

pub struct Tetris {
    width: i32,
    height: i32,
    cells: Vec<u8>,
    active_piece: Piece,
}

impl Tetris {
    pub fn new(width: i32, height: i32) -> Tetris {
        let cells = (0..width * height).map(|_| 0).collect();

        Tetris {
            width: width,
            height: height,
            cells: cells,
            active_piece: Piece::new(PieceKind::TShape),
        }
    }

    pub fn tick(&mut self) {
        if self.wont_collide(&self.active_piece.pre_drop()) {
            self.piece_disintegrate();
            self.active_piece.drop();
            self.piece_active_integrate();
        } else {
            self.piece_disintegrate();
            self.piece_integrate();
            self.piece_add(Piece::new(PieceKind::TShape));
            self.piece_active_integrate();
        }
            
    }

    pub fn piece_left(&mut self) {
        self.piece_disintegrate();
        if self.wont_collide(&self.active_piece.pre_left()) {
            self.active_piece.left();
        }
        self.piece_active_integrate();
    }
    
    pub fn piece_right(&mut self) {
        self.piece_disintegrate();
        if self.wont_collide(&self.active_piece.pre_right()) {
            self.active_piece.right();
        }
        self.piece_active_integrate();
    }

    pub fn piece_rotate_clockwise(&mut self) {
        self.piece_disintegrate();
        if self.wont_collide(&self.active_piece.pre_rotate_right()) {
            self.active_piece.rotate_right();
        }
        self.piece_active_integrate();
    }
    
    pub fn cells(&self) -> *const u8 {
        self.cells.as_ptr()
    }
}

/// Private functions.
impl Tetris {
    fn piece_integrate(&mut self) {
        for block in self.active_piece.blocks.iter() {
            let index = ((self.width * (block.y)) + (block.x)) as usize;
            self.cells[index] = 1;
        }
    }
    
    fn piece_active_integrate(&mut self) {
        for block in self.active_piece.blocks.iter() {
            let index = ((self.width * (block.y)) + (block.x)) as usize;
            self.cells[index] = 2;
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
                block.y < (self.height) &&
                (self.cells[self.get_index(&block)] == 0 ||
                self.cells[self.get_index(&block)] == 2)
            }
        )
    }

    fn get_index(&self, block: &Block) -> usize {
        ((self.width * block.y) + block.x) as usize
    }

    pub fn draw(&self, canvas: Canvas, block_color: &str) {
        for col in 0..self.width {
            for row in 0..self.height {
                let idx = self.get_index(&Block{x: col, y: row});

                let cell_color = match self.cells[idx] {
                    0 => "white",
                    _ => block_color,
                };

                canvas.draw_block(col as u32, row as u32, cell_color);
            }
        }

    }
}

impl fmt::Display for Tetris {
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
