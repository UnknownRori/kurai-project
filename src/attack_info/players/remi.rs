use std::sync::Arc;

use hecs::World;
use macroquad::texture::Texture2D;

use crate::{
    assets::konst::REMI_BULLET_1,
    cmpx,
    components::{attack_info::AttackSpawner, movement::MoveParams},
    engine::{assets::AssetsManager, components::Transform2D},
    entity::spawn_player_bullet,
};

#[derive(Debug)]
pub struct RemiliaBasicAttack {
    bullet: Arc<Texture2D>,
}

impl RemiliaBasicAttack {
    pub fn new(assets: &AssetsManager) -> Self {
        let bullet = assets.textures.get(REMI_BULLET_1).unwrap();

        Self { bullet }
    }
}

impl AttackSpawner for RemiliaBasicAttack {
    fn spawn(&self, world: &mut World, current: &Transform2D, _: &Transform2D, bullet_speed: f32) {
        spawn_player_bullet(
            world,
            current,
            Arc::clone(&self.bullet),
            MoveParams::move_linear(cmpx!(0., -bullet_speed)),
        );
    }
}
