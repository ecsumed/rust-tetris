#![allow(dead_code)]
#![allow(unreachable_patterns)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
struct Block {
    x: i32,
    y: i32,
}

impl Block {
    fn negate(&self) -> Block {
        Block {
            x: -self.x,
            y: -self.y,
        }
    }
}

enum PieceKind {
    long,
    tshape,
    lshape,
    rlshape,    // reversed 'L' shape
    sshape,
    rsshape,    //reversed 'S' shape
}

pub struct Piece {
    blocks: Vec<Block>,
}

impl Piece {
    fn new(kind: PieceKind) -> Result<Piece, &'static str> {
        let blocks = match kind {
            PieceKind::long => [(0, 1), (0, 0), (0, 2), (0, 3)],
            PieceKind::tshape => [(0, 1), (0, 0), (1, 1), (0, 2)],
            PieceKind::lshape => [(0, 1), (0, 0), (0, 2), (1, 2)],
            PieceKind::rlshape => [(1, 1), (1, 0), (1, 2), (0, 2)],
            PieceKind::sshape => [(1, 1), (0, 1), (0, 0), (1, 2)],
            PieceKind::rsshape => [(1, 1), (1, 0), (0, 1), (0, 2)],
            _ => return Err("Piece kind not found!"),
        };
        Ok(Piece {
            blocks: blocks.iter().map(|x| Block { x: x.0, y: x.1 }).collect(),
        })
    }

    fn pre_drop(&self) -> Vec<Block> {
        self.blocks.iter().map(|b| Block{x: b.x, y: b.y + 1}).collect()
    }
    
    fn drop(&mut self) {
        self.blocks = self.blocks.iter().map(|b| Block{x: b.x, y: b.y + 1}).collect()
    }
    
    fn right(&mut self) {
        self.blocks = self.blocks.iter().map(|b| Block{x: b.x + 1, y: b.y}).collect()
    }
    
    fn left(&mut self) {
        self.blocks = self.blocks.iter().map(|b| Block{x: b.x - 1, y: b.y}).collect()
    }
    
    fn get_floor_blocks(&self) -> Vec<Block> {
        let mut floor_blocks = HashMap::new();
    
        for block in self.blocks.iter() {
            match floor_blocks.insert(block.x, block.y) {
                Some(v) => {
                    if v > block.y {
                        floor_blocks.insert(block.x, v);
                    }
                },
                _ => (),
            }
        }  
       floor_blocks.iter().map(|(k, v)| Block{x: *k, y: *v} ).collect()
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, block) in self.blocks.iter().enumerate() {
            write!(f, "#{}) x: {}, y: {}\n", i + 1, block.x, block.y)?;
        }
        Ok(())
    }
}

struct Canvas {
    width: i32,
    height: i32,
    cells: Vec<u8>,
    active_piece: Option<Piece>,
}

impl Canvas {
    fn new(width: i32, height: i32) -> Canvas {
        let cells = (0..width * height).map(|i| 0).collect();
        Canvas {
            width: width,
            height: height,
            cells: cells,
            active_piece: None,
        }
    }

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

    fn piece_add(&mut self, piece: Piece) {
        self.active_piece = Some(piece);
    }
    
    fn tick(&mut self) {
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

mod geometry {
    use Block;
    use Piece;

    pub fn rotate_l(piece: &mut Piece) {
        let degree_90 = -90.0_f64.to_radians();
        let rotated_piece = transpose_piece(&piece, &piece.blocks[0]);
        let rotated_piece = rotate_piece(&rotated_piece, degree_90);
        let rotated_piece = untranspose_piece(&rotated_piece, &piece.blocks[0]);
        std::mem::replace(piece, rotated_piece);
    }
    
    pub fn rotate_r(piece: &mut Piece) {
        let degree_90 = 90.0_f64.to_radians();
        let rotated_piece = transpose_piece(&piece, &piece.blocks[0]);
        let rotated_piece = rotate_piece(&rotated_piece, degree_90);
        let rotated_piece = untranspose_piece(&rotated_piece, &piece.blocks[0]);
        std::mem::replace(piece, rotated_piece);
    }

    fn rotate_piece(piece: &Piece, angle: f64) -> Piece {
        Piece {
            blocks: piece
                .blocks
                .iter()
                .map(|point| rotate_point(&point, angle))
                .collect(),
        }
    }

    fn transpose_piece(piece: &Piece, center: &Block) -> Piece {
        Piece {
            blocks: piece
                .blocks
                .iter()
                .map(|point| transpose_point(&point, center))
                .collect(),
        }
    }

    fn untranspose_piece(piece: &Piece, center: &Block) -> Piece {
        Piece {
            blocks: piece
                .blocks
                .iter()
                .map(|point| transpose_point(&point, &center.negate()))
                .collect(),
        }
    }

    fn rotate_point(point: &Block, angle: f64) -> Block {
        Block {
            x: -(angle.sin() * (point.y as f64)) as i32,
            y: (angle.sin() * (point.x as f64)) as i32,
        }
    }

    fn transpose_point(point: &Block, center: &Block) -> Block {
        Block {
            x: point.x - center.x,
            y: point.y - center.y,
        }
    }
}

fn main() {
    let mut lp = Piece::new(PieceKind::tshape).unwrap();

    let mut frame = Canvas::new(10, 20);
    
    frame.piece_add(lp);
    frame.tick();

    println!("{}", frame);
}

