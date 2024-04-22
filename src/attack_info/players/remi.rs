use hecs::World;
use macroquad::texture::Texture2D;

use crate::{
    assets::{konst::BULLET_REMI, Assets},
    cmpx,
    components::{AttackSpawner, MoveParams, Transform2D},
    entity::player_bullet_spawn,
    time::Timer,
};

#[derive(Debug, Clone)]
pub struct RemiliaBasicAttack {
    bullet: Texture2D,
    timer: Timer,
}

impl RemiliaBasicAttack {
    pub fn new(assets: &Assets) -> Self {
        let bullet = assets.textures.get(BULLET_REMI).unwrap().clone();
        let timer = Timer::new(0.05, true);

        Self { bullet, timer }
    }
}

impl AttackSpawner for RemiliaBasicAttack {
    fn spawn(&mut self, world: &mut World, current: &Transform2D, _: Option<&Transform2D>) {
        self.timer.update();
        if self.timer.completed() {
            player_bullet_spawn(
                world,
                &self.bullet,
                current.clone(),
                MoveParams::move_linear(cmpx!(0., -2.)),
            )
        }
    }
}
