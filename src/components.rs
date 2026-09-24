#[derive(Debug)]

// location data
pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

// stats
pub struct Health {
    pub hp: i32
}

pub struct Defense {
    pub def: i32
}

pub struct Attack {
    pub atk: i32
}

pub struct Dead;

pub struct Player;
pub struct Enemy;