use std::{collections::VecDeque, sync::Arc};

use crate::math::*;
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
    pub max_speed: f32,
}

#[derive(Debug)]
pub struct TargetPlayer;

#[derive(Debug)]
pub struct SingleShoot;

#[derive(Default, Debug, Clone, Copy)]
pub struct EnemyBullet {
    graze: bool,
}

impl EnemyBullet {
    pub fn is_grazed(&self) -> bool {
        self.graze
    }

    pub fn grazed(&mut self) {
        self.graze = true;
    }
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

#[derive(Debug)]
pub struct Hitpoint {
    pub hp: f32,
    pub max_hp: f32,
    pub invulnerable: bool, // INFO : Phase for invulnerable stuff
}

impl Hitpoint {
    pub fn new(hp: f32) -> Self {
        Self {
            hp,
            max_hp: hp,
            invulnerable: false,
        }
    }

    pub fn damage(&mut self, damage: f32) -> bool {
        self.hp -= damage;
        self.hp <= 0.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Hitbox {
    pub radius: f32, // Normalized value from size of sprite
}

impl Hitbox {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }

    pub fn is_intersect(
        &self,
        current_pos: &Position,
        target_pos: &Position,
        target_hitbox: &Hitbox,
    ) -> bool {
        let distance_squared = current_pos
            .position
            .to_vec2()
            .distance_squared(target_pos.position.to_vec2());
        let sum_of_radii_squared = (self.radius + target_hitbox.radius).powi(2);
        distance_squared <= sum_of_radii_squared
    }

    pub fn near(
        &self,
        current_pos: &Position,
        target_pos: &Position,
        target_hitbox: &Hitbox,
    ) -> bool {
        let distance_squared = current_pos
            .position
            .to_vec2()
            .distance_squared(target_pos.position.to_vec2());
        let sum_of_radii_squared = (self.radius + 0.05 + target_hitbox.radius).powi(2);
        distance_squared <= sum_of_radii_squared
    }

    pub fn draw(&self, position: &Position, screen: &Window) {
        let pos = position.position.to_vec2();
        let real_pos: Vec2 = pos.reset_from_vec2(*screen.playable_window().size())
            + *screen.playable_window().get_start();
        let size = self.radius * screen.playable_window().size().y;
        draw_circle(real_pos.x, real_pos.y, size, RED);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CanShoot {
    pub fire_rate: f64,
    pub last_shoot: Option<Instant>,
    pub bullet_speed: f32,
}

impl Default for CanShoot {
    fn default() -> Self {
        Self {
            fire_rate: 1.0,
            bullet_speed: 0.1,
            last_shoot: None,
        }
    }
}

impl CanShoot {
    #[must_use]
    pub fn new(firerate: f64, speed: f32) -> Self {
        Self {
            fire_rate: firerate,
            bullet_speed: speed,
            last_shoot: None,
        }
    }

    #[must_use]
    pub fn can_fire(&self, time_frame: f64) -> bool {
        if self.last_shoot.is_none() {
            return true;
        }

        self.last_shoot.unwrap().elapsed(time_frame) >= 1.0 / self.fire_rate
    }

    #[must_use]
    pub fn update_cooldown(&mut self) {
        self.last_shoot = Some(Instant::now());
    }
}

#[derive(Clone)]
pub struct Sprite {
    pub texture: Arc<Texture2D>,
    pub size: Vec2,
}

impl Sprite {
    pub fn new(texture: Arc<Texture2D>, size: Vec2) -> Self {
        Self { texture, size }
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
        let pos = position.position.to_vec2() - self.size / 2.0;
        let real_pos: Vec2 = pos.reset_from_vec2(*screen.playable_window().size())
            + *screen.playable_window().get_start();
        let real_size = self.size.reset_from_vec2(*screen.playable_window().size());

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
    pub done: Option<Instant>,  // INFO : Only used when it's smooth
    pub smooth: bool,
}

#[derive(Debug)]
pub struct MovementQueue {
    pub start: Instant,
    pub target_move: VecDeque<Movement>,
}

impl Movement {
    pub fn new(target: Complex<f32>, wait: f64, smooth: bool) -> Self {
        Self {
            target,
            wait,
            start: None,
            done: None,
            smooth,
        }
    }

    pub fn start(&mut self, delta: f64) {
        self.start = Some(Instant::new(delta));
    }

    pub fn dir(&self, current_pos: &Complex<f32>, move_speed: f32, delta: f32) -> Complex<f32> {
        (self.target - current_pos).normalize() * move_speed * delta
    }
}

impl MovementQueue {
    pub fn new(target_move: Vec<Movement>) -> Self {
        Self {
            start: Instant::now(),
            target_move: target_move.into(),
        }
    }

    pub fn pop(&mut self) {
        self.target_move.pop_front();
    }
}
