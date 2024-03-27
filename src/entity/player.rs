use std::sync::Arc;

use hecs::World;
use keyframe::functions::EaseInOut;
use macroquad::math::vec2;

use crate::{
    assets::konst::{FOCUS, REMILIA_TEXTURE_1},
    attack_info::players::remi::RemiliaBasicAttack,
    components::{
        attack_info::{AttackInfo, AttackSpawner, GenericAttackInfo, PlayerAttack, SpellInfo},
        player::{Focus, Player},
        velocity::{AcceleratedVelocity, DampedVelocity, Velocity},
    },
    engine::{
        assets::AssetsManager,
        components::{CircleHitbox2D, Sprite2D, Transform2D},
        math::complx,
    },
};

pub fn lazy_spawn_player(assets_manager: &AssetsManager) -> Box<dyn Fn(&mut World)> {
    let texture = assets_manager.textures.get(REMILIA_TEXTURE_1).unwrap();
    let focus = assets_manager.textures.get(FOCUS).unwrap();

    // TODO : Put this character specific somewhere
    let remi_attack = Arc::new(RemiliaBasicAttack::new(assets_manager)) as Arc<dyn AttackSpawner>;

    Box::new(move |world| {
        let remi_attack = remi_attack.clone();
        let player_attack = PlayerAttack {
            normal: AttackInfo::new(GenericAttackInfo::new(2., 20.), Arc::clone(&remi_attack)),
            focus: AttackInfo::new(GenericAttackInfo::new(2., 20.), Arc::clone(&remi_attack)),
            spell: SpellInfo::new(
                1,
                "Gugnir".to_string(),
                0.,
                Arc::clone(&remi_attack),
                GenericAttackInfo::new(20., 1.),
            ),
        };
        let focus = Focus(Sprite2D::new(focus.clone()));

        world.spawn((
            Player,
            Transform2D::new(complx(0.5, 0.5), vec2(0.1, 0.1), 0.),
            focus,
            DampedVelocity(25.),
            Velocity::Normal(complx(0., 0.)),
            AcceleratedVelocity::new(1.2, 0.8, 0.8, 0.5, EaseInOut),
            Sprite2D::new(texture.clone()),
            player_attack,
            CircleHitbox2D::new(0.010),
        ));
    })
}
