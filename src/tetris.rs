use std::fmt;

use canvas::Canvas;
use piece::Block;
use piece::Piece;
use piece::PieceKind;

#[derive(Clone)]
pub struct Tetris {
    width: i32,
    height: i32,
    cells: Vec<u8>,
    active_piece: Piece,
}

// Public functions
impl Tetris {
    pub fn new(width: i32, height: i32) -> Tetris {
        let cells = (0..width * height).map(|_| 0).collect();

        Tetris {
            width: width,
            height: height,
            cells: cells,
            active_piece: Piece::new(PieceKind::random()),
        }
    }

    pub fn tick(&mut self) {
        self.piece_disintegrate();
        if self.wont_collide(&self.active_piece.pre_drop()) {
            self.active_piece.drop();
        } else {
            if self.at_top(&self.active_piece.blocks) {
                // reset grid, game over
                self.cells = (0..self.width * self.height).map(|_| 0).collect();
            } else {
                self.piece_integrate();
                self.remove_full_rows();
            }
            self.piece_add(Piece::new(PieceKind::random()));
        }
        self.piece_active_integrate();
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

    pub fn draw(&self, canvas: &Canvas) {
        for col in 0..self.width {
            for row in 0..self.height {
                let idx = self.get_index(&Block { x: col, y: row });

                let cell_color = match self.cells[idx] {
                    0 => "white",
                    1 => "#5C7D87",
                    2 => "#29A49A",
                    3 => "#BC2A5D",
                    4 => "#E48537",
                    5 => "#ef3636",
                    6 => "#9f25c8",
                    7 => "#8BD03E",
                    8 => "#5CB3FF",
                    _ => "black",
                };

                canvas.draw_block(col as u32, row as u32, cell_color);
            }
        }
    }
}

// Private functions.
impl Tetris {
    fn piece_integrate(&mut self) {
        for block in self.active_piece.blocks.iter() {
            let index = ((self.width * (block.y)) + (block.x)) as usize;
            let p_type = match self.active_piece.kind {
                PieceKind::Long => 1,
                PieceKind::TShape => 2,
                PieceKind::LShape => 3,
                PieceKind::RLShape => 4,
                PieceKind::SShape => 5,
                PieceKind::RSShape => 6,
                PieceKind::BShape => 7,
            };
            self.cells[index] = p_type;
        }
    }

    fn piece_active_integrate(&mut self) {
        for block in self.active_piece.blocks.iter() {
            let index = ((self.width * (block.y)) + (block.x)) as usize;
            self.cells[index] = 8; // 0 for no block + 7 enum types = 8 for active block
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
        block.iter().all(|block| {
            block.x >= 0
                && block.x < (self.width)
                && block.y >= 0
                && block.y < (self.height)
                && (self.cells[self.get_index(&block)] == 0
                    || self.cells[self.get_index(&block)] == 8)
        })
    }

    fn at_top(&self, blocks: &Vec<Block>) -> bool {
        blocks.iter().any(|block| block.y == 0)
    }

    fn get_index(&self, block: &Block) -> usize {
        ((self.width * block.y) + block.x) as usize
    }

    fn remove_full_rows(&mut self) {
        let mut temp = self.cells.clone();
        for (row, cells) in self.cells.chunks(self.width as usize).enumerate() {
            if cells.iter().all(|&x| (x >= 1 && x <= 7)) {
                let start = self.width as usize * row;
                let end = start + self.width as usize;
                temp.drain(start..end);
                temp.splice(..0, (0..10).map(|_| 0));
            }
        }
        self.cells = temp;
    }

    fn cells(&self) -> *const u8 {
        self.cells.as_ptr()
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
