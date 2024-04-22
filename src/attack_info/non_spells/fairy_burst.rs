use macroquad::prelude::*;

use crate::{
    assets::{konst::BULLET_RED, Assets},
    components::{AttackSpawner, Bullet, Enemy, Hitbox, MoveParams, Sprite, Transform2D},
    math::ComplexExt,
    time::Timer,
    vec2,
};

#[derive(Debug)]
pub struct FairyBurst {
    bullet: Texture2D,
    timer: Timer,
}

impl FairyBurst {
    pub fn new(assets: &Assets, delay_between_shot: f32) -> Self {
        let bullet = assets.textures.get(BULLET_RED).unwrap().clone();
        let timer = Timer::new(delay_between_shot, true);

        Self { bullet, timer }
    }
}

impl AttackSpawner for FairyBurst {
    fn spawn(
        &mut self,
        world: &mut hecs::World,
        current: &Transform2D,
        player: Option<&Transform2D>,
    ) {
        self.timer.update();
        if !self.timer.completed() {
            return;
        }

        match player {
            Some(player) => {
                // Get the direction from target position (player)
                let dir = current.position().dir(player.position());

                let transform = Transform2D {
                    scale: vec2!(0.03, 0.03),
                    rotation: dir.rot(),
                    ..*current
                };

                let component = (
                    Enemy,
                    Bullet,
                    transform,
                    Hitbox::new(0.010),
                    Sprite::new(self.bullet.clone()),
                    MoveParams::move_linear(dir),
                );

                world.spawn(component);
            }
            None => {}
        }
    }
}
