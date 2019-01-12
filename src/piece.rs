use std::fmt;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug)]
pub struct Block {
    pub x: i32,
    pub y: i32,
}

impl Block {
    fn negate(&self) -> Block {
        Block {
            x: -self.x,
            y: -self.y,
        }
    }
}

pub enum PieceKind {
    Long,       // Long 'I' shape
    TShape,     // 'T' shape
    LShape,     // 'L' shape
    RLShape,    // reversed 'L' shape
    SShape,     // 'S' shape
    RSShape,    //reversed 'S' shape
}

impl Distribution<PieceKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceKind {
        match rng.gen_range(0, 5) {
            0 => PieceKind::Long,
            1 => PieceKind::TShape,
            2 => PieceKind::LShape,
            3 => PieceKind::RLShape,
            4 => PieceKind::SShape,
            _ => PieceKind::RSShape,
        }
    }
}
pub struct Piece {
    pub blocks: Vec<Block>,
}

impl Piece {
    pub fn new(kind: PieceKind) -> Result<Piece, &'static str> {
        let blocks = match kind {
            PieceKind::Long => [(0, 1), (0, 0), (0, 2), (0, 3)],
            PieceKind::TShape => [(0, 1), (0, 0), (1, 1), (0, 2)],
            PieceKind::LShape => [(0, 1), (0, 0), (0, 2), (1, 2)],
            PieceKind::RLShape => [(1, 1), (1, 0), (1, 2), (0, 2)],
            PieceKind::SShape => [(1, 1), (0, 1), (0, 0), (1, 2)],
            PieceKind::RSShape => [(1, 1), (1, 0), (0, 1), (0, 2)],
        };
        Ok(Piece {
            blocks: blocks.iter().map(|x| Block { x: x.0, y: x.1 }).collect(),
        })
    }

    pub fn pre_drop(&self) -> Vec<Block> {
        self.blocks.iter().map(|b| Block{x: b.x, y: b.y + 1}).collect()
    }
    
    pub fn drop(&mut self) {
        self.blocks = self.blocks.iter().map(|b| Block{x: b.x, y: b.y + 1}).collect()
    }
    
    pub fn pre_right(&self) -> Vec<Block> {
        self.blocks.iter().map(|b| Block{x: b.x + 1, y: b.y}).collect()
    }
    
    pub fn right(&mut self) {
        self.blocks = self.blocks.iter().map(|b| Block{x: b.x + 1, y: b.y}).collect()
    }
    
    pub fn pre_left(&self) -> Vec<Block> {
        self.blocks.iter().map(|b| Block{x: b.x - 1, y: b.y}).collect()
    }
    
    pub fn left(&mut self) {
        self.blocks = self.blocks.iter().map(|b| Block{x: b.x - 1, y: b.y}).collect()
    }
    
    pub fn pre_up(&self) -> Vec<Block> {
        self.blocks.iter().map(|b| Block{x: b.x, y: b.y - 1}).collect()
    }
    
    pub fn up(&mut self) {
        self.blocks = self.blocks.iter().map(|b| Block{x: b.x, y: b.y - 1}).collect()
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
