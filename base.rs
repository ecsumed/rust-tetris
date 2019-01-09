#![allow(dead_code)]
#![allow(unreachable_patterns)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
struct Block {
    x: u8,
    y: u8,
}

enum PieceKind {
    long,
    tshape,
    lshape,
    rlshape,    // reversed 'L' shape
    sshape,
    rsshape,    //reversed 'S' shape
}

struct Piece {
    blocks: Vec<Block>,
}

impl Piece {
    fn new(kind: PieceKind) -> Result<Piece, &'static str> {
        let blocks = match kind {
            PieceKind::long => [(0, 0), (0, 1), (0, 2), (0, 3)],
            PieceKind::tshape => [(0, 0), (0, 1), (1, 1), (0, 2)],
            PieceKind::lshape => [(0, 0), (0, 1), (0, 2), (1, 2)],
            PieceKind::rlshape => [(1, 0), (1, 1), (1, 2), (0, 2)],
            PieceKind::sshape => [(0, 0), (0, 1), (1, 1), (1, 2)],
            PieceKind::rsshape => [(1, 0), (1, 1), (0, 1), (0, 2)],
            _ => return Err("Piece kind not found!"),
        };
        Ok(Piece {
            blocks: blocks.iter().map(|x| Block { x: x.0, y: x.1 }).collect(),
        })
    }

    fn move_down(&mut self) {
        self.blocks = self.blocks.iter().map(|b| Block{x: b.x, y: b.y + 1}).collect()
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
    width: u64,
    height: u64,
    cells: Vec<u8>,
    active_piece: Option<Piece>,
}

impl Canvas {
    fn new(width: u64, height: u64) -> Canvas {
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
                let index = ((self.width as u8 * (block.y)) + (block.x)) as usize;
                // println!("x: {}, y:{}", block.x, block.y);
                // println!("index: {}", index);
                self.cells[index] = 1;
            }
        }
    }
    
    fn piece_disintegrate(&mut self) {
        if let Some(piece) = &self.active_piece {
            for block in piece.blocks.iter() {
                let index = ((self.width as u8 * (block.y)) + (block.x)) as usize;
                self.cells[index] = 0;
            }
        }
    }

    fn piece_add(&mut self, piece: Piece) {
        self.active_piece = Some(piece);
    }
    
    fn tick(&mut self) {
        self.piece_disintegrate();
    
        
        if let Some(piece) = &mut self.active_piece {
            piece.move_down();
            let floor_blocks = piece.get_floor_blocks();
            println!("{:?}", floor_blocks);
        }
        self.piece_integrate();
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

fn main() {
    let lp = Piece::new(PieceKind::rsshape).unwrap();

    let mut frame = Canvas::new(10, 20);

    frame.piece_add(lp);
    frame.tick();

    println!("{}", frame);
}

