use hecs::World;

use crate::{
    components::{attack_info::AttackInfo, bullet::Bullet, enemy::Enemy, player::Player},
    engine::components::Transform2D,
};

pub fn attack_info_trigger(world: &mut World, time: f64, delta: f32) {
    let players = world
        .query::<(&Player, &Transform2D)>()
        .without::<&Bullet>()
        .iter()
        .map(|(_, (_, transform))| (transform.clone()))
        .collect::<Vec<_>>();

    if players.len() < 1 {
        return;
    }

    let enemies = world
        .query::<(&Enemy, &mut AttackInfo, &mut Transform2D)>()
        .iter()
        .map(|(id, (_, info, transform))| (id.clone(), info.clone(), transform.clone()))
        .collect::<Vec<_>>();

    let player_transform = players.first().unwrap();
    for (id, info, transform) in enemies {
        let mut spawner = info.spawner.lock().unwrap();
        spawner.spawn(world, &transform, player_transform, delta);
    }
}
