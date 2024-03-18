use hecs::World;

use crate::{
    components::{bullet::Bullet, enemy::Enemy, player::Player},
    engine::components::{CircleHitbox2D, Transform2D},
};

pub fn collision_player_with_enemy_bullets(world: &mut World) {
    let players = world
        .query::<(&Player, &Transform2D, &CircleHitbox2D)>()
        .without::<&Bullet>()
        .iter()
        .map(|(id, (_, transform, hitbox))| (id.clone(), transform.clone(), hitbox.clone()))
        .collect::<Vec<_>>();

    let enemy_bullets = world
        .query::<(&Enemy, &Bullet, &Transform2D, &CircleHitbox2D)>()
        .iter()
        .map(|(id, (_, _, transform, hitbox))| (id.clone(), transform.clone(), hitbox.clone()))
        .collect::<Vec<_>>();

    for (player_id, player_transform, player_hitbox) in players {
        for (bullet_id, bullet_transform, bullet_hitbox) in &enemy_bullets {
            if player_hitbox.is_intersect_with_circle_hitbox(
                &player_transform,
                &bullet_transform,
                bullet_hitbox,
            ) {
                // INFO : Update Score!
                world.despawn(*bullet_id).unwrap();
            } else if player_hitbox.near(&player_transform, &bullet_transform, bullet_hitbox) {
                // INFO : Update Score!
                println!("{:#?} - Graze! - {:#?}", player_id, bullet_id);
            }
        }
    }
}
