use geometry;
use std::fmt;

use stdweb::js;
use stdweb::unstable::TryInto;

#[derive(Debug, Clone)]
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

#[derive(Clone)]
pub enum PieceKind {
    Long,    // Long 'I' shape
    TShape,  // 'T' shape
    LShape,  // 'L' shape
    RLShape, // reversed 'L' shape
    SShape,  // 'S' shape
    RSShape, // reversed 'S' shape
    BShape,  // Box shape
}

impl PieceKind {
    pub fn random() -> PieceKind {
        let num = PieceKind::random_int(7).try_into().unwrap();

        match num {
            0 => PieceKind::Long,
            1 => PieceKind::TShape,
            2 => PieceKind::LShape,
            3 => PieceKind::RLShape,
            4 => PieceKind::SShape,
            5 => PieceKind::RSShape,
            6 => PieceKind::BShape,
            _ => PieceKind::TShape,
        }
    }

    fn random_int(max: i32) -> stdweb::Value {
        let val = js! {
            return Math.floor(Math.random() * Math.floor(@{max}));
        };
        return val;
    }
}

#[derive(Clone)]
pub struct Piece {
    pub blocks: Vec<Block>,
    pub kind: PieceKind,
}

impl Piece {
    pub fn new(kind: PieceKind) -> Piece {
        let blocks = match kind {
            PieceKind::Long => [(0, 1), (0, 0), (0, 2), (0, 3)],
            PieceKind::TShape => [(0, 1), (0, 0), (1, 1), (0, 2)],
            PieceKind::LShape => [(0, 1), (0, 0), (0, 2), (1, 2)],
            PieceKind::RLShape => [(1, 1), (1, 0), (1, 2), (0, 2)],
            PieceKind::SShape => [(1, 1), (0, 0), (0, 1), (1, 2)],
            PieceKind::RSShape => [(1, 1), (1, 0), (0, 1), (0, 2)],
            PieceKind::BShape => [(0, 0), (0, 1), (1, 0), (1, 1)],
        };

        Piece {
            blocks: blocks.iter().map(|x| Block { x: x.0, y: x.1 }).collect(),
            kind: kind,
        }
    }

    pub fn pre_drop(&self) -> Vec<Block> {
        self.blocks
            .iter()
            .map(|b| Block { x: b.x, y: b.y + 1 })
            .collect()
    }

    pub fn drop(&mut self) {
        self.blocks = self
            .blocks
            .iter()
            .map(|b| Block { x: b.x, y: b.y + 1 })
            .collect()
    }

    pub fn pre_right(&self) -> Vec<Block> {
        self.blocks
            .iter()
            .map(|b| Block { x: b.x + 1, y: b.y })
            .collect()
    }

    pub fn right(&mut self) {
        self.blocks = self
            .blocks
            .iter()
            .map(|b| Block { x: b.x + 1, y: b.y })
            .collect()
    }

    pub fn pre_left(&self) -> Vec<Block> {
        self.blocks
            .iter()
            .map(|b| Block { x: b.x - 1, y: b.y })
            .collect()
    }

    pub fn left(&mut self) {
        self.blocks = self
            .blocks
            .iter()
            .map(|b| Block { x: b.x - 1, y: b.y })
            .collect()
    }

    pub fn pre_up(&self) -> Vec<Block> {
        self.blocks
            .iter()
            .map(|b| Block { x: b.x, y: b.y - 1 })
            .collect()
    }

    pub fn up(&mut self) {
        self.blocks = self
            .blocks
            .iter()
            .map(|b| Block { x: b.x, y: b.y - 1 })
            .collect()
    }

    pub fn pre_rotate_right(&self) -> Vec<Block> {
        let degree_90 = 90.0_f64.to_radians();
        let rotated_piece = Piece::transpose(self, &self.blocks[0]);
        let rotated_piece = Piece::rotate(&rotated_piece, degree_90);
        Piece::transpose(&rotated_piece, &self.blocks[0].negate()).blocks
    }

    pub fn rotate_right(&mut self) {
        let degree_90 = 90.0_f64.to_radians();
        let rotated_piece = Piece::transpose(self, &self.blocks[0]);
        let rotated_piece = Piece::rotate(&rotated_piece, degree_90);
        let rotated_piece = Piece::transpose(&rotated_piece, &self.blocks[0].negate());
        std::mem::replace(self, rotated_piece);
    }

    fn rotate(piece: &Piece, angle: f64) -> Piece {
        Piece {
            blocks: piece
                .blocks
                .iter()
                .map(|point| geometry::rotate_point(&point, angle))
                .collect(),
            kind: piece.kind.clone(),
        }
    }

    fn transpose(piece: &Piece, center: &Block) -> Piece {
        Piece {
            blocks: piece
                .blocks
                .iter()
                .map(|point| geometry::transpose_point(&point, center))
                .collect(),
            kind: piece.kind.clone(),
        }
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
