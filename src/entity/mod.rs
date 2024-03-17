use std::sync::Arc;

use hecs::{Entity, World};
use macroquad::{math::vec2, texture::Texture2D};

use crate::{
    assets::konst::{FOCUS, REMILIA_TEXTURE_1},
    attack_info::remi::RemiliaBasicAttack,
    components::{
        attack_info::{AttackInfo, AttackSpawner, GenericAttackInfo, PlayerAttack, SpellInfo},
        bullet::Bullet,
        enemy::Enemy,
        player::{Focus, Player},
    },
    engine::{
        assets::AssetsManager,
        components::{
            CircleHitbox2D, Hitpoint, Movable, Movement, MovementQueue, Sprite2D, Transform2D,
            Velocity,
        },
        math::{complx, ComplexExt},
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
            Movable::new(1., 1.),
            Sprite2D::new(texture.clone()),
            player_attack,
        ));
    })
}

// pub fn spawn_player(world: &mut World, assets_manager: &AssetsManager) {
//     let texture = assets_manager.textures.get(REMILIA_TEXTURE_1).unwrap();
//     let focus = assets_manager.textures.get(FOCUS).unwrap();
//
//     // TODO : Put this character specific somewhere
//     let remi_attack = Arc::new(RemiliaBasicAttack::new(assets_manager)) as Arc<dyn AttackSpawner>;
//     let player_attack = PlayerAttack {
//         normal: AttackInfo::new(GenericAttackInfo::new(2., 20.), Arc::clone(&remi_attack)),
//         focus: AttackInfo::new(GenericAttackInfo::new(2., 20.), Arc::clone(&remi_attack)),
//         spell: SpellInfo::new(
//             1,
//             "Gugnir".to_string(),
//             0.,
//             Arc::clone(&remi_attack),
//             GenericAttackInfo::new(20., 1.),
//         ),
//     };
//
//     world.spawn((
//         Player,
//         Transform2D::new(complx(0.5, 0.5), vec2(0.1, 0.1), 0.),
//         Focus(Sprite2D::new(focus)),
//         Movable::new(1., 1.),
//         Sprite2D::new(texture),
//         player_attack,
//     ));
// }

pub fn spawn_player_bullet(
    world: &mut World,
    transform: &Transform2D,
    texture: Arc<Texture2D>,
    velocity: Velocity,
) -> Entity {
    let transform = Transform2D {
        scale: vec2(0.15, 0.15),
        ..*transform
    };

    let component = (
        Player,
        Bullet,
        transform,
        Movable::default(),
        CircleHitbox2D::new(0.010),
        Sprite2D::new(texture),
        velocity,
    );

    world.spawn(component)
}

pub fn lazy_spawn_enemy(
    transform: Transform2D,
    texture: Arc<Texture2D>,
    movement: Vec<Movement>,
    hitpoint: Hitpoint,
) -> Box<dyn Fn(&mut World)> {
    Box::new(move |world| {
        world.spawn((
            Enemy,
            transform,
            Movable::new(0.2, 0.4),
            Sprite2D::new(texture.clone()),
            MovementQueue::new(movement.clone()),
            hitpoint.clone(),
        ));
    })
}

pub fn spawn_enemy(
    world: &mut World,
    transform: Transform2D,
    texture: Arc<Texture2D>,
    movement: MovementQueue,
    hitpoint: Hitpoint,
) {
    world.spawn((
        Enemy,
        transform,
        Movable::new(0.2, 0.4),
        Sprite2D::new(texture),
        movement,
        hitpoint,
    ));
}

// TODO : Not completed
pub fn spawn_generic_bullet(
    world: &mut World,
    current: &Transform2D,
    target: &Transform2D,
    velocity: Velocity,
    texture: Arc<Texture2D>,
) -> Entity {
    let speed = 0.5;
    let direction = (target.position - current.position).normalize() * speed;
    let rot = direction.conj().arg() - std::f32::consts::FRAC_PI_2;

    world.spawn((
        Enemy,
        Bullet,
        Sprite2D::new(texture),
        Transform2D::new(current.position, vec2(0.1, 0.1), rot),
        CircleHitbox2D::new(0.004),
        velocity,
    ))
}
