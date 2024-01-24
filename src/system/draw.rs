use hecs::World;
use macroquad::prelude::*;
use rayon::prelude::*;

use crate::entity::{PlayerBulletEntity, PlayerEntity};

pub fn update_render_player_bullet(world: &World) {
    for (_, (_, position, _, _)) in &mut world.query::<PlayerBulletEntity>() {
        draw_circle(position.position.x, position.position.y, 5.0, GRAY);
    }
}

pub fn update_render_player(world: &World) {
    world
        .query::<PlayerEntity>()
        .iter()
        .for_each(|(_, (_, _, _, position, _, _))| {
            draw_rectangle(position.position.x, position.position.y, 10.0, 10.0, BLUE);
        });
}
