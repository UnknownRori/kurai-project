use hecs::World;
use macroquad::prelude::*;

use crate::{
    assets::konst::REMI_BULLET_1,
    cmpx,
    components::{attack_info::AttackSpawner, movement::MoveParams},
    engine::{
        assets::AssetsManager,
        components::{Sprite2D, Transform2D},
    },
    entity::spawn_player_bullet,
};

#[derive(Debug)]
pub struct RemiliaBasicAttack {
    bullet: Texture2D,
}

impl RemiliaBasicAttack {
    pub fn new(assets: &AssetsManager) -> Self {
        let bullet = assets.textures.get(REMI_BULLET_1).unwrap();

        Self { bullet }
    }
}

impl AttackSpawner for RemiliaBasicAttack {
    fn spawn(
        &self,
        world: &mut World,
        current: &Transform2D,
        _: &Transform2D,
        bullet_speed: f32,
        _delta: f32,
    ) {
        spawn_player_bullet(
            world,
            current,
            Sprite2D::new(self.bullet.clone()),
            MoveParams::move_linear(cmpx!(0., -bullet_speed)),
        );
    }
}
