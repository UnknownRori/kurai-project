use hecs::World;
use macroquad::prelude::*;

use crate::{
    controls::{Action, Controls},
    entity::PlayerEntity,
    window::Window,
};

/// Register all the system related stuff of ECS here
pub fn update_system(world: &mut World, controls: &Controls, screen: &Window) {
    update_player_move(world, controls, screen);
}

fn update_player_move(world: &mut World, controls: &Controls, screen: &Window) {
    for (_, (_, _, moveable, position, _)) in &mut world.query::<PlayerEntity>() {
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
    }
}
