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
