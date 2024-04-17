use std::sync::{Arc, Mutex};

use hecs::World;

use crate::{
    assets::konst::{FOCUS, REMILIA_TEXTURE_1},
    assets::AssetsManager,
    attack_info::players::remi::RemiliaBasicAttack,
    cmpx,
    components::{
        attack_info::{AttackInfo, AttackSpawnFn, PlayerAttack, SpellInfo},
        circle_hitbox2d::CircleHitbox2D,
        movement::MoveParams,
        player::{Focus, Player},
        sprite2d::Sprite2D,
        transform2d::Transform2D,
    },
    vec2,
};

pub fn lazy_spawn_player(assets_manager: &AssetsManager) -> Box<dyn Fn(&mut World)> {
    let texture = assets_manager.textures.get(REMILIA_TEXTURE_1).unwrap();
    let focus = assets_manager.textures.get(FOCUS).unwrap();

    // TODO : Put this character specific somewhere
    let remi_attack =
        Arc::new(Mutex::new(RemiliaBasicAttack::new(assets_manager))) as AttackSpawnFn;

    Box::new(move |world| {
        let remi_attack = remi_attack.clone();
        let player_attack = PlayerAttack {
            normal: AttackInfo::new(Arc::clone(&remi_attack)),
            focus: AttackInfo::new(Arc::clone(&remi_attack)),
            spell: SpellInfo::new(1, "Gugnir".to_string(), 0., Arc::clone(&remi_attack)),
        };
        let focus = Focus(Sprite2D::new(focus.clone()));

        world.spawn((
            Player,
            Transform2D::new(cmpx!(0.5), vec2!(0.1), 0.),
            focus,
            MoveParams::move_dampen(cmpx!(0.), 0.85),
            Sprite2D::new(texture.clone()),
            player_attack,
            CircleHitbox2D::new(0.010),
        ));
    })
}
