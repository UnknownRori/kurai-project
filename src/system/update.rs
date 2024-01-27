use std::time::Instant;

use hecs::World;
use macroquad::prelude::*;
use num_complex::Complex;
use rayon::prelude::*;

use crate::components::CanShoot;
use crate::math::*;

use crate::{
    components::{EnemyBullet, Movable, Velocity},
    controls::{Action, Controls},
    entity::{spawn_player_bullet, BulletEntity, NormalFairyEntity, PlayerEntity},
    window::Window,
};

pub fn enemy_shoot_normal_fairy(world: &mut World) {
    let player_pos = world
        .query::<PlayerEntity>()
        .iter()
        .par_bridge()
        .map(|(_, (_, _, _, pos, _, _))| (*pos))
        .collect::<Vec<_>>();

    if let Some(player_pos) = player_pos.first() {
        let enemy = world
            .query::<NormalFairyEntity>()
            .iter()
            .par_bridge()
            .filter(|(_, (_, _, _, can_shoot, _, _))| can_shoot.can_fire())
            .map(|(entity, (_, pos, _, can_shoot, _, _))| (entity, *pos, *can_shoot))
            .collect::<Vec<_>>();

        for (entity, pos, can_shoot) in enemy {
            let direction =
                (player_pos.position - pos.position).normalize() * can_shoot.bullet_speed;

            world.spawn((
                EnemyBullet,
                pos.clone(),
                Movable::default(),
                Velocity::from(direction),
            ));

            let _ = world
                .get::<&mut CanShoot>(entity)
                .unwrap()
                .update_cooldown();
        }
    };
}

pub fn player_shoot(world: &mut World, controls: &Controls) {
    // TODO : Make sure remove the clone since it's not efficient
    let player_entity = world
        .query::<PlayerEntity>()
        .iter()
        .par_bridge()
        .map(|(_, (_, _, _, pos, can_shoot, sprite))| (*pos, *can_shoot, sprite.clone()))
        .collect::<Vec<_>>();

    for (pos, can_shoot, sprite) in &player_entity {
        if controls.is_down(&Action::Attack) {
            let pos = pos.position + Complex::new(0.0, 0.0);

            spawn_player_bullet(
                world,
                &pos.into(),
                Complex::new(0.0, -can_shoot.bullet_speed),
            );
        }
    }
}

pub fn update_delete_bullet_offscreen(world: &mut World, screen: &Window) {
    // TODO : Fix this later
    let out_of_bound_bullet = world
        .query::<BulletEntity>()
        .iter()
        .par_bridge()
        .filter(|(_, (pos, _, _))| {
            pos.position.im > *screen.get_height()
                || pos.position.im < 0.0
                || pos.position.re < 0.0
                || pos.position.re > *screen.get_width()
        })
        .map(|(id, _)| id)
        .collect::<Vec<_>>();

    for id in out_of_bound_bullet {
        world.despawn(id).unwrap();
    }
}

pub fn update_move_bullet(world: &mut World, _screen: &Window) {
    world.query_mut::<BulletEntity>().into_iter().for_each(
        |(_, (position, moveable, velocity))| {
            position.position += velocity.velocity * get_frame_time();
        },
    );
}

pub fn update_player_move(world: &World, controls: &Controls, screen: &Window) {
    world
        .query::<PlayerEntity>()
        .iter()
        .for_each(|(_, (_, _, moveable, position, _, _))| {
            let mut new_pos = Complex::new(0.0, 0.0);

            if controls.is_down(&Action::Left) {
                new_pos += Complex::new(-moveable.move_speed, 0.0);
            }

            if controls.is_down(&Action::Right) {
                new_pos += Complex::new(moveable.move_speed, 0.0);
            }

            if controls.is_down(&Action::Up) {
                new_pos += Complex::new(0.0, -moveable.move_speed);
            }

            if controls.is_down(&Action::Down) {
                new_pos += Complex::new(0.0, moveable.move_speed);
            }

            position.position += new_pos * get_frame_time();
            position.position.re = position
                .position
                .re
                .clamp(-32.0, screen.get_playable_window().get_end_width() - 32.0);
            position.position.im = position
                .position
                .im
                .clamp(-32.0, *screen.get_playable_window().get_end_height() - 32.0);
        });
}
