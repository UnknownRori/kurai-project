use hecs::World;
use macroquad::prelude::*;
use rayon::prelude::*;

use crate::{
    controls::{Action, Controls},
    entity::{spawn_player_bullet, PlayerBulletEntity, PlayerEntity},
    window::Window,
};

/// Register all the system related stuff of ECS here
pub fn update_system(world: &mut World, controls: &Controls, screen: &Window) {
    update_player_move(world, controls, screen);
    update_render_player_bullet(world);
    update_move_bullet(world, screen);
    player_shoot(world, controls);
}

fn player_shoot(world: &mut World, controls: &Controls) {
    // TODO : Make sure remove the clone since it's not efficient
    let player_entity = world
        .query::<PlayerEntity>()
        .iter()
        .par_bridge()
        .map(|(_, (_, _, _, pos, can_shoot, _))| (pos.clone(), can_shoot.clone()))
        .collect::<Vec<_>>();

    for (pos, can_shoot) in player_entity.iter() {
        if controls.is_down(&Action::Attack) {
            let a = spawn_player_bullet(
                world,
                &pos,
                Vec2::from_array([0.0, -can_shoot.bullet_speed]),
            );

            tracing::info!("Outside : {:#?}", a);
        }
    }
}

fn update_move_bullet(world: &mut World, screen: &Window) {}

fn update_render_player_bullet(world: &mut World) {
    // TODO : Not query properly
    for (a, (_, position, _, _)) in &mut world.query::<PlayerBulletEntity>() {
        draw_circle(position.position.x, position.position.y, 5.0, GRAY);
    }
}

fn update_player_move(world: &mut World, controls: &Controls, screen: &Window) {
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

            // Normalize the new position and set the current entity into new position
            new_pos = new_pos.normalize_or_zero();
            new_pos.x *= moveable.move_speed;
            new_pos.y *= moveable.move_speed;
            position.position.x += new_pos.x;
            position.position.y += new_pos.y;
            position.position = position.position.clamp(
                Vec2::from_array([0.0, 0.0]),
                Vec2::from_array([*screen.get_width() - 10.0, *screen.get_height() - 10.0]),
            );

            // Draw the player using dummy rectangle
            // TODO : Extract this to different component or system or something...
            draw_rectangle(position.position.x, position.position.y, 10.0, 10.0, BLUE);
        });
}
