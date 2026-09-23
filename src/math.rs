use crate::components::Position;

pub fn distance(a: &Position, b: &Position) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;

    (dx * dx + dy * dy).sqrt()
}

pub fn sign(current: f32, target: f32) -> f32 {
    if (current - target) > 0.0 {
        return -1.0;
    }
    if (current - target) < 0.0 {
        return 1.0;
    }
    else {
        0.0
    }
}