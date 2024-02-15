use crate::assets::{AssetsHandler, AssetsManager};
use crate::score::ScoreData;
use crate::time::Instant;

use hecs::World;
use macroquad::prelude::*;
use num_complex::Complex;
use rayon::prelude::*;

use crate::components::{CanShoot, Enemy, Hitbox, Player, Position};
use crate::entity::{spawn_generic_bullet, EnemyMovableEntity};
use crate::math::*;

use crate::{
    components::{EnemyBullet, Movable, Velocity},
    controls::{Action, Controls},
    entity::{spawn_player_bullet, BulletEntity, NormalFairyEntity, PlayerEntity},
    window::Window,
};

pub fn enemy_shoot_normal_fairy(
    world: &mut World,
    assets_manager: &AssetsManager,
    delta: f32,
    time: f64,
) {
    let player_pos = world
        .query::<PlayerEntity>()
        .iter()
        .par_bridge()
        .map(|(_, (_, _, _, pos, _, _, _))| (*pos))
        .collect::<Vec<_>>();

    if let Some(player_pos) = player_pos.first() {
        let enemy = world
            .query::<NormalFairyEntity>()
            .iter()
            .par_bridge()
            .filter(move |(_, (_, pos, _, can_shoot, _, _, _, _, _))| {
                can_shoot.can_fire(time)
                    && ((pos.position.re >= 0.050 && pos.position.re <= 0.950)
                        && (pos.position.im >= 0.050 && pos.position.im <= 0.950))
            })
            .map(|(entity, (_, pos, _, can_shoot, _, _, _, _, _))| (entity, *pos, *can_shoot))
            .collect::<Vec<_>>();

        for (entity, pos, can_shoot) in enemy {
            let texture = assets_manager
                .textures
                .get("bullet0")
                .expect("No generic bullet texture!");
            spawn_generic_bullet(world, &pos, player_pos, can_shoot.bullet_speed, texture);

            let _ = world
                .get::<&mut CanShoot>(entity)
                .unwrap()
                .update_cooldown();
        }
    };
}

pub fn enemy_movement_update(world: &mut World, delta: f32, time: f64) {
    world
        .query::<EnemyMovableEntity>()
        .iter()
        .par_bridge()
        .for_each(|(_, (_, pos, moveable, movement_queue))| {
            if let Some(current_queue) = movement_queue.target_move.get_mut(0) {
                if current_queue.start.is_none() {
                    current_queue.start = Some(Instant::new(time));
                }

                let distance = (pos.position - current_queue.target).norm();
                let tolerance = 0.005;
                if distance > tolerance {
                    pos.position += current_queue.dir(&pos.position, moveable.move_speed, delta);
                } else {
                    if current_queue.done.is_none() {
                        current_queue.done = Some(Instant::new(time))
                    } else if current_queue.done.unwrap().elapsed(time) > current_queue.wait {
                        movement_queue.pop();
                    }
                }
            }
        });
}

pub fn player_shoot(
    world: &mut World,
    assets_manager: &AssetsManager,
    controls: &Controls,
    _: f32,
    time: f64,
) {
    // TODO : Make sure remove the clone since it's not efficient
    let player_entity = world
        .query::<PlayerEntity>()
        .iter()
        .par_bridge()
        .map(|(entity, (_, _, _, pos, can_shoot, _, _))| (entity, *pos, *can_shoot))
        .collect::<Vec<_>>();

    for (entity, pos, can_shoot) in player_entity {
        if controls.is_down(&Action::Attack) && can_shoot.can_fire(time) {
            let pos = pos.position + Complex::new(0.0, 0.0);

            // TODO : Use proper player texture
            let texture = assets_manager
                .textures
                .get("remi-bullet-0")
                .expect("Generic bullet is not found");
            spawn_player_bullet(
                world,
                &pos.into(),
                texture,
                Complex::new(0.0, -can_shoot.bullet_speed),
            );

            let _ = world
                .get::<&mut CanShoot>(entity)
                .unwrap()
                .update_cooldown();
        }
    }
}

pub fn update_collision_detection_enemy_bullet_to_player(world: &mut World, score: &mut ScoreData) {
    let player_entity = world
        .query::<(&Player, &Position, &Hitbox)>()
        .iter()
        .map(|(_, (_, pos, hitbox))| (*pos, hitbox.clone()))
        .collect::<Vec<_>>();

    let enemy_bullet = world
        .query::<(&EnemyBullet, &Position, &Hitbox)>()
        .iter()
        .par_bridge()
        .map(|(entity, (enemy_bullet, pos, hitbox))| {
            (
                entity.clone(),
                enemy_bullet.clone(),
                pos.clone(),
                hitbox.clone(),
            )
        })
        .collect::<Vec<_>>();

    for (player_pos, player_hitbox) in player_entity {
        for (enemy_entity, enemy_bullet, enemy_pos, enemy_hitbox) in &enemy_bullet {
            if player_hitbox.is_intersect(&player_pos, &enemy_pos, &enemy_hitbox) {
                world.despawn(*enemy_entity).unwrap();
                score.life -= 1;
                score.graze -= 1; // TODO : Make this something more interesting
            } else if player_hitbox.near(&player_pos, &enemy_pos, &enemy_hitbox)
                && !enemy_bullet.is_grazed()
            {
                score.graze += 1;
                world
                    .get::<&mut EnemyBullet>(*enemy_entity)
                    .unwrap()
                    .grazed();
            }
        }
    }
}

pub fn update_delete_bullet_offscreen(world: &mut World, screen: &Window, _: f32, _: f64) {
    // TODO : Fix this later
    let out_of_bound_bullet = world
        .query::<BulletEntity>()
        .iter()
        .par_bridge()
        .filter(|(_, (pos, _, _, _, _))| {
            pos.position.re <= 0.0
                || pos.position.re >= 1.0
                || pos.position.im <= 0.0
                || pos.position.im >= 1.0
        })
        .map(|(id, _)| id)
        .collect::<Vec<_>>();

    for id in out_of_bound_bullet {
        world.despawn(id).unwrap();
    }
}

pub fn update_move_bullet(world: &mut World, _screen: &Window, delta: f32, _: f64) {
    world
        .query_mut::<BulletEntity>()
        .into_iter()
        .for_each(|(_, (position, _, velocity, _, _))| {
            position.position += velocity.velocity * delta;
            position.position = position
                .position
                .clamp(Complex::new(0.0, 0.0), Complex::new(1.00, 1.00));
        });
}

pub fn update_player_move(world: &World, controls: &Controls, screen: &Window, _: f32, _: f64) {
    world
        .query::<PlayerEntity>()
        .iter()
        .for_each(|(_, (_, _, moveable, position, _, _, _))| {
            let mut new_pos = Complex::new(0.0, 0.0);
            let move_speed = if controls.is_down(&Action::Focus) {
                moveable.move_speed / 2.
            } else {
                moveable.move_speed
            };

            if controls.is_down(&Action::Left) {
                new_pos += Complex::new(-move_speed, 0.0);
            }

            if controls.is_down(&Action::Right) {
                new_pos += Complex::new(move_speed, 0.0);
            }

            if controls.is_down(&Action::Up) {
                new_pos += Complex::new(0.0, -move_speed);
            }

            if controls.is_down(&Action::Down) {
                new_pos += Complex::new(0.0, move_speed);
            }

            position.position += new_pos * get_frame_time();
            position.position = position
                .position
                .clamp(Complex::new(0.05, 0.05), Complex::new(0.95, 0.95));
        });
}
