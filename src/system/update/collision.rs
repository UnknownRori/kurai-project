use hecs::World;

use crate::{
    components::{
        bullet::{Bullet, GrazedBullet},
        enemy::Enemy,
        player::Player,
    },
    engine::components::{CircleHitbox2D, Transform2D},
    score::ScoreData,
};

pub fn collision_player_with_enemy_bullets(world: &mut World, score: &mut ScoreData) {
    let players = world
        .query::<(&Player, &Transform2D, &CircleHitbox2D)>()
        .without::<&Bullet>()
        .iter()
        .map(|(id, (_, transform, hitbox))| (id.clone(), transform.clone(), hitbox.clone()))
        .collect::<Vec<_>>();

    let enemy_bullets = world
        .query::<(
            &Enemy,
            &Bullet,
            Option<&GrazedBullet>,
            &Transform2D,
            &CircleHitbox2D,
        )>()
        .iter()
        .map(|(id, (_, _, grazed, transform, hitbox))| {
            (
                id.clone(),
                grazed.is_some(),
                transform.clone(),
                hitbox.clone(),
            )
        })
        .collect::<Vec<_>>();

    for (player_id, player_transform, player_hitbox) in players {
        for (bullet_id, is_grazed, bullet_transform, bullet_hitbox) in &enemy_bullets {
            if player_hitbox.is_intersect_with_circle_hitbox(
                &player_transform,
                &bullet_transform,
                bullet_hitbox,
            ) {
                world.despawn(*bullet_id).unwrap();
                score.life -= 1;
                score.graze -= 1; // TODO : Make this something more interesting
                                  // TODO : Added death animation and despawn player
            } else if player_hitbox.near(&player_transform, &bullet_transform, bullet_hitbox) {
                // INFO : Update Score!
                println!("{:#?} - Graze! - {:#?}", player_id, bullet_id);

                if !is_grazed {
                    world.insert_one(*bullet_id, GrazedBullet).unwrap();
                    score.graze += 1
                }
            }
        }
    }
}
