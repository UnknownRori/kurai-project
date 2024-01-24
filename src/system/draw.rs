use hecs::World;
use macroquad::prelude::*;

use crate::{
    entity::{PlayerBulletEntity, PlayerEntity},
    shaders::shadow_shader_material,
    window::Window,
};

pub fn update_render_player_bullet(world: &World) {
    for (_, (_, position, _, _)) in &mut world.query::<PlayerBulletEntity>() {
        draw_circle(position.position.x, position.position.y, 5.0, GRAY);
    }
}

pub fn update_render_player(world: &World, screen: &Window) {
    world
        .query::<PlayerEntity>()
        .iter()
        .for_each(|(_, (_, _, _, position, _, sprite))| {
            // INFO : Not working currently | causing the game to crash and burn
            // let material = shadow_shader_material(&screen).unwrap();
            // gl_use_material(&material);
            draw_texture_ex(
                &sprite.texture,
                position.position.x,
                position.position.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(64.0, 64.0)),
                    ..Default::default()
                },
            );
            // gl_use_default_material();
        });
}
