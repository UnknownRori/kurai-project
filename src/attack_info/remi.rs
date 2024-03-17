use std::sync::Arc;

use hecs::World;
use macroquad::texture::Texture2D;

use crate::{
    assets::konst::REMI_BULLET_1,
    components::attack_info::AttackSpawner,
    engine::{assets::AssetsManager, components::Transform2D, math::complx},
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
    fn spawn(&self, world: &mut World, current: &Transform2D, bullet_speed: f32) {
        spawn_player_bullet(
            world,
            current,
            Arc::clone(&self.bullet),
            crate::engine::components::Velocity::Normal(complx(0., -bullet_speed)),
        );
    }
}
