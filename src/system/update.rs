use hecs::World;
use macroquad::prelude::*;
use rayon::prelude::*;

use crate::{
    components::{Movable, Position, Velocity},
    controls::{Action, Controls},
    entity::{spawn_player_bullet, PlayerEntity},
    window::Window,
};

pub fn player_shoot(world: &mut World, controls: &Controls) {
    // TODO : Make sure remove the clone since it's not efficient
    let player_entity = world
        .query::<PlayerEntity>()
        .iter()
        .par_bridge()
        .map(|(_, (_, _, _, pos, can_shoot, _))| (*pos, *can_shoot))
        .collect::<Vec<_>>();

    for (pos, can_shoot) in &player_entity {
        if controls.is_down(&Action::Attack) {
            // INFO : Spawning the bullet
            spawn_player_bullet(world, pos, Vec2::from_array([0.0, -can_shoot.bullet_speed]));
        }
    }
}

pub fn update_move_bullet(world: &mut World, _screen: &Window) {
    world
        .query_mut::<(&mut Position, &Movable, &Velocity)>()
        .into_iter()
        .for_each(|(_, (position, moveable, velocity))| {
            position.position = position.position.lerp(
                position.position + velocity.velocity,
                moveable.move_speed * get_frame_time(),
            );
        });
}

pub fn update_player_move(world: &World, controls: &Controls, screen: &Window) {
    world
        .query::<PlayerEntity>()
        .iter()
        .for_each(|(_, (_, _, moveable, position, _, _))| {
            let mut new_pos = Vec2::new(0.0, 0.0);

            if controls.is_down(&Action::Left) {
                new_pos.x -= moveable.move_speed;
            }

            if controls.is_down(&Action::Right) {
                new_pos.x += moveable.move_speed;
            }

            if controls.is_down(&Action::Up) {
                new_pos.y -= moveable.move_speed;
            }

            if controls.is_down(&Action::Down) {
                new_pos.y += moveable.move_speed;
            }

            position.position = position
                .position
                .lerp(
                    new_pos + position.position,
                    moveable.move_speed * get_frame_time(),
                )
                .clamp(
                    Vec2::from_array([0.0, 0.0]),
                    Vec2::from_array([*screen.get_width() - 10.0, *screen.get_height() - 10.0]),
                )
        });
}
