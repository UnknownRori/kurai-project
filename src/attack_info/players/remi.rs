use hecs::World;
use macroquad::prelude::*;

use crate::{
    assets::konst::REMI_BULLET_1,
    cmpx,
    components::{attack_info::AttackSpawner, movement::MoveParams},
    engine::{
        assets::AssetsManager,
        components::{Sprite2D, Transform2D},
        time::Timer,
    },
    entity::spawn_player_bullet,
};

#[derive(Debug, Clone)]
pub struct RemiliaBasicAttack {
    bullet: Texture2D,
    timer: Timer,
}

impl RemiliaBasicAttack {
    pub fn new(assets: &AssetsManager) -> Self {
        let bullet = assets.textures.get(REMI_BULLET_1).unwrap();
        let timer = Timer::new(0.05, true);

        Self { bullet, timer }
    }
}

impl AttackSpawner for RemiliaBasicAttack {
    fn spawn(&mut self, world: &mut World, current: &Transform2D, _: &Transform2D, _delta: f32) {
        self.timer.update();
        if self.timer.completed() {
            spawn_player_bullet(
                world,
                current,
                Sprite2D::new(self.bullet.clone()),
                MoveParams::move_linear(cmpx!(0., -2.)),
            );
        }
    }
}
