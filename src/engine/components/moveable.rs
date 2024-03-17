#[derive(Debug)]
pub struct Movable {
    pub move_speed: f32,
    pub max_speed: f32,
}

impl Default for Movable {
    fn default() -> Self {
        Self {
            move_speed: 0.01,
            max_speed: 0.05,
        }
    }
}

impl Movable {
    #[must_use]
    pub const fn new(move_speed: f32, max_speed: f32) -> Self {
        Self {
            move_speed,
            max_speed,
        }
    }
}
