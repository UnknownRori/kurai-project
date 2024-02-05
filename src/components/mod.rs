use std::sync::Arc;

use crate::{math::ToVec2, time::Instant, window::Window};
use macroquad::prelude::*;
use num_complex::Complex;

#[derive(Debug)]
pub struct Player;

#[derive(Debug)]
pub struct Enemy;

#[derive(Debug)]
pub struct PlayerBullet;

#[derive(Debug)]
pub struct Controllable;

#[derive(Debug)]
pub struct Movable {
    pub move_speed: f32,
}

#[derive(Debug)]
pub struct TargetPlayer;

#[derive(Debug)]
pub struct SingleShoot;

#[derive(Debug)]
pub struct EnemyBullet;

impl Default for Movable {
    fn default() -> Self {
        Self { move_speed: 0.01 }
    }
}

impl Movable {
    #[must_use]
    pub const fn new(move_speed: f32) -> Self {
        Self { move_speed }
    }
}

#[derive(Debug)]
pub struct DummyDraw;

#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    pub position: Complex<f32>,
}

impl Position {
    #[must_use]
    #[inline]
    pub const fn from_array(arr: [f32; 2]) -> Self {
        Self {
            position: Complex::new(arr[0], arr[1]),
        }
    }
}

impl From<Complex<f32>> for Position {
    fn from(value: Complex<f32>) -> Self {
        Self { position: value }
    }
}

#[derive(Debug, Default)]
pub struct Velocity {
    pub velocity: Complex<f32>,
}

impl From<Complex<f32>> for Velocity {
    fn from(value: Complex<f32>) -> Self {
        Self { velocity: value }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CanShoot {
    pub fire_rate: f64,
    pub last_shoot: Instant,
    pub bullet_speed: f32,
}

impl Default for CanShoot {
    fn default() -> Self {
        Self {
            fire_rate: 1.0,
            bullet_speed: 0.1,
            last_shoot: Instant::now(),
        }
    }
}

impl CanShoot {
    #[must_use]
    pub fn new(firerate: f64, speed: f32) -> Self {
        Self {
            fire_rate: firerate,
            bullet_speed: speed,
            last_shoot: Instant::now(),
        }
    }

    #[must_use]
    pub fn can_fire(&self, time_frame: f64) -> bool {
        self.last_shoot.elapsed(time_frame) >= 1.0 / self.fire_rate
    }

    #[must_use]
    pub fn update_cooldown(&mut self) {
        self.last_shoot = Instant::now();
    }
}

#[derive(Clone)]
pub struct Sprite {
    pub texture: Arc<Texture2D>,
}

impl Sprite {
    pub fn new(texture: Arc<Texture2D>) -> Self {
        Self { texture }
    }

    #[must_use]
    pub fn height(&self) -> f32 {
        self.texture.height()
    }

    #[must_use]
    pub fn width(&self) -> f32 {
        self.texture.width()
    }

    #[must_use]
    pub fn draw(&self, position: &Position, screen: &Window) {
        let size = vec2(0.1, 0.1);
        let pos = position.position.to_vec2() - size / 2.0;
        let real_pos: Vec2 =
            pos * (*screen.playable_window().size()) + (*screen.playable_window().get_start());
        let real_size = size * (*screen.playable_window().size());

        draw_texture_ex(
            &self.texture,
            real_pos.x,
            real_pos.y,
            WHITE,
            DrawTextureParams {
                // TODO : Make sure it looks good on any screens
                dest_size: Some(real_size),
                ..Default::default()
            },
        );
    }
}

#[derive(Debug)]
pub struct Movement {
    pub target: Complex<f32>,
    pub wait: f64,
    pub start: Option<Instant>, // INFO : Only used when it's smooth
    pub smooth: bool,
}

#[derive(Debug)]
pub struct MovementQueue {
    pub start: Instant,
    pub target_move: Vec<Movement>,
}

impl Movement {
    pub fn new(target: Complex<f32>, wait: f64, smooth: bool) -> Self {
        Self {
            target,
            wait,
            start: None,
            smooth,
        }
    }

    fn start(&mut self, delta: f64) {
        self.start = Some(Instant::new(delta));
    }
}

impl MovementQueue {
    pub fn new(wait: f64, target_move: Vec<Movement>) -> Self {
        Self {
            start: Instant::now(),
            target_move,
        }
    }
}
