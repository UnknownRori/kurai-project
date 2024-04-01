use std::sync::Arc;

use macroquad::{math::vec2, texture::Texture2D};

use crate::{
    assets::konst::RED_BULLET,
    components::{attack_info::AttackSpawner, bullet::Bullet, enemy::Enemy, movement::MoveParams},
    engine::{
        assets::AssetsManager,
        components::{CircleHitbox2D, Sprite2D, Transform2D},
        math::ComplexExt,
    },
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
        // Get the direction from target position (player)
        let dir = current.position().dir(player.as_ref());

        let transform = Transform2D {
            scale: vec2(0.03, 0.03),
            rotation: dir.rot(),
            ..*current
        };

        let component = (
            Enemy,
            Bullet,
            transform,
            CircleHitbox2D::new(0.010),
            Sprite2D::new(Arc::clone(&self.bullet)),
            MoveParams::move_linear(dir),
            // Velocity::Normal(cmpx!(0., 0.)),
            // ConstantAcceleration::new(bullet_speed, dir, 1., 12., EaseInOutCubic),
        );

        world.spawn(component);
    }
}
