use piece::Block;

pub fn rotate_point(point: &Block, angle: f64) -> Block {
    Block {
        x: -(angle.sin() * (point.y as f64)) as i32,
        y: (angle.sin() * (point.x as f64)) as i32,
    }
}

pub fn transpose_point(point: &Block, center: &Block) -> Block {
    Block {
        x: point.x - center.x,
        y: point.y - center.y,
    }
}
