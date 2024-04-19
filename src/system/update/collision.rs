use hecs::World;
use macroquad::time::get_frame_time;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    components::{
        bullet::{Bullet, GrazedBullet},
        circle_hitbox2d::CircleHitbox2D,
        death::{Death, DeathBlinkingAnimation},
        enemy::Enemy,
        hitpoint::{Damaged, Hitpoint},
        player::Player,
        transform2d::Transform2D,
    },
    score::ScoreData,
};

pub fn collision_player_with_enemy_bullets(world: &mut World, score: &mut ScoreData) {
    {
        // TODO : Put this somewhere
        world
            .query::<(&Death, &mut DeathBlinkingAnimation)>()
            .iter()
            .for_each(|(_, (_, blink))| {
                blink.update();
            });
    }

    {
        let done_animate = world
            .query::<(&Death, &DeathBlinkingAnimation)>()
            .iter()
            .filter(|(_, (_, blink))| blink.done())
            .map(|(entity, (_, _))| (entity))
            .collect::<Vec<_>>();

        for entity in done_animate {
            world
                .remove::<(Death, DeathBlinkingAnimation)>(entity)
                .unwrap();
        }
    }

    {
        let players = world
            .query::<(&Player, &Transform2D, &CircleHitbox2D)>()
            .without::<&Bullet>()
            .without::<&Death>()
            .without::<&DeathBlinkingAnimation>()
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
            .par_bridge()
            .map(|(id, (_, _, grazed, transform, hitbox))| {
                (
                    id.clone(),
                    grazed.is_some(),
                    transform.clone(),
                    hitbox.clone(),
                )
            })
            .collect::<Vec<_>>();

        for (id, player_transform, player_hitbox) in players {
            let mut already_hit = false;
            for (bullet_id, is_grazed, bullet_transform, bullet_hitbox) in &enemy_bullets {
                if player_hitbox.is_intersect_with_circle_hitbox(
                    &player_transform,
                    &bullet_transform,
                    bullet_hitbox,
                ) && !already_hit
                {
                    already_hit = true;
                    world.despawn(*bullet_id).unwrap();
                    score.life -= 1;
                    score.graze -= 1; // TODO : Make this something more interesting
                                      // TODO : Added death animation and despawn player
                    world
                        .insert(id, (Death, DeathBlinkingAnimation::default()))
                        .unwrap();
                } else if player_hitbox.near(&player_transform, &bullet_transform, bullet_hitbox) {
                    // INFO : Update Score!

                    if !is_grazed {
                        world.insert_one(*bullet_id, GrazedBullet).unwrap();
                        score.graze += 1
                    }
                }
            }
        }
    }

    {
        let player_bullets = world
            .query::<(&Player, &Bullet, &Transform2D, &CircleHitbox2D)>()
            .iter()
            .map(|(id, (_, _, transform, hitbox))| (id.clone(), transform.clone(), hitbox.clone()))
            .collect::<Vec<_>>();

        let enemies = world
            .query::<(&Enemy, &Hitpoint, &Transform2D, &CircleHitbox2D)>()
            .without::<&Bullet>()
            .iter()
            .map(|(id, (_, _, transform, hitbox))| (id.clone(), transform.clone(), hitbox.clone()))
            .collect::<Vec<_>>();

        for (id, transform, hitbox) in enemies {
            // TODO : Deal damage based on bullet
            for (bullet_id, bullet_transform, bullet_hitbox) in &player_bullets {
                if bullet_hitbox.is_intersect_with_circle_hitbox(
                    &bullet_transform,
                    &transform,
                    &hitbox,
                ) {
                    world.get::<&mut Hitpoint>(id).unwrap().damage(0.5);

                    if !world.satisfies::<&Damaged>(id).unwrap() {
                        println!("Doesn't have damaged!");
                        world.insert(id, (Damaged,)).unwrap();
                    } else {
                        println!("Satisfies");
                    }

                    // if world.get::<&Hitpoint>(id).unwrap().is_dead() {
                    //     world.despawn(id).unwrap();
                    // }

                    world.despawn(*bullet_id).unwrap();
                }
            }
        }
    }
}
