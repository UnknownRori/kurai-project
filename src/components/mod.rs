use macroquad::math::Vec2;

#[derive(Debug)]
pub struct Player;

#[derive(Debug)]
pub struct Controllable;

#[derive(Debug)]
pub struct Movable {
    pub move_speed: f32,
}

impl Default for Movable {
    fn default() -> Self {
        Self { move_speed: 10.0 }
    }
}

#[derive(Debug)]
pub struct DummyDraw;

#[derive(Debug, Default)]
pub struct Position {
    pub position: Vec2,
}
