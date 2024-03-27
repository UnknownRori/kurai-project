use std::sync::Arc;

use macroquad::{math::vec2, texture::Texture2D};

use crate::{
    assets::konst::RED_BULLET,
    components::{attack_info::AttackSpawner, velocity::Velocity},
    engine::{assets::AssetsManager, components::Transform2D, math::ComplexExt},
    entity::bullet::spawn_generic_bullet,
};

#[derive(Debug)]
pub struct FairyBurst {
    bullet: Arc<Texture2D>,
}

impl FairyBurst {
    pub fn new(assets: &AssetsManager) -> Self {
        let bullet = assets.textures.get(RED_BULLET).unwrap();

        Self { bullet }
    }
}

impl AttackSpawner for FairyBurst {
    fn spawn(
        &self,
        world: &mut hecs::World,
        current: &crate::engine::components::Transform2D,
        player: &crate::engine::components::Transform2D,
        bullet_speed: f32,
    ) {
        // Get the direction from target position (player) and add some speed
        let velocity = current.position().dir(player.as_ref()) * bullet_speed;

        let transform = Transform2D {
            scale: vec2(0.03, 0.03),
            rotation: velocity.rot(),
            ..*current
        };
        spawn_generic_bullet(
            world,
            transform,
            Velocity::Normal(velocity),
            Arc::clone(&self.bullet),
        );
    }
}
