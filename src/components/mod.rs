use std::{sync::Arc, time::Instant};

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
        Self { move_speed: 20.0 }
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
    pub fire_rate: f32,
    pub last_shoot: Instant, // INFO : This is not work for wasm
    pub bullet_speed: f32,
}

impl Default for CanShoot {
    fn default() -> Self {
        Self {
            fire_rate: 1.0,
            bullet_speed: 20.0,
            last_shoot: Instant::now(),
        }
    }
}

impl CanShoot {
    pub fn new(firerate: f32, speed: f32) -> Self {
        Self {
            fire_rate: firerate,
            bullet_speed: speed,
            last_shoot: Instant::now(),
        }
    }

    pub fn can_fire(&self) -> bool {
        self.last_shoot.elapsed().as_secs_f32() >= 1.0 / self.fire_rate
    }

    pub fn update_cooldown(&mut self) {
        self.last_shoot = Instant::now();
    }
}

#[derive(Clone)]
pub struct Sprite {
    pub texture: Arc<Texture2D>,
}

impl Sprite {
    pub async fn new() -> Self {
        // TODO : Put this thing away
        let texture = load_texture("./resources/textures/remilia-scarlet/1.png")
            .await
            .unwrap()
            .into();

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
}
