use hecs::World;
use macroquad::prelude::*;

use crate::{
    entity::{NormalFairyBulletEntity, NormalFairyEntity, PlayerBulletEntity, PlayerEntity},
    window::Window,
};

pub fn update_render_player_bullet(world: &World) {
    for (_, (_, position, _, _)) in &mut world.query::<PlayerBulletEntity>() {
        draw_circle(position.position.re, position.position.im, 5.0, GRAY);
    }
}

pub fn update_render_enemy(world: &World) {
    world
        .query::<NormalFairyEntity>()
        .iter()
        .for_each(|(_, (_, pos, _, _, _, _))| {
            draw_rectangle(pos.position.re, pos.position.im, 10.0, 10.0, RED);
        });
}

pub fn update_render_normal_fairy_bullet(world: &World) {
    world
        .query::<NormalFairyBulletEntity>()
        .iter()
        .for_each(|(_, (_, pos, _, _))| draw_circle(pos.position.re, pos.position.im, 5.0, RED));
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
                position.position.re - 32.0,
                position.position.im - 32.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(64.0, 64.0)),
                    ..Default::default()
                },
            );
            // gl_use_default_material();
        });
}
