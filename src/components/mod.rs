use macroquad::math::Vec2;

#[derive(Debug)]
pub struct Player;

#[derive(Debug)]
pub struct PlayerBullet;

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

#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    pub position: Vec2,
}

impl Position {
    fn from_array(arr: [f32; 2]) -> Self {
        Self {
            position: Vec2::from_array(arr),
        }
    }
}

#[derive(Debug, Default)]
pub struct Velocity {
    pub velocity: Vec2,
}

impl Velocity {
    pub fn from_vec2(velocity: Vec2) -> Self {
        Self { velocity }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CanShoot {
    pub shoot_speed: f32,
    pub bullet_speed: f32,
}

impl Default for CanShoot {
    fn default() -> Self {
        Self {
            shoot_speed: 1.0,
            bullet_speed: 20.0,
        }
    }
}
