use hecs::World;
use macroquad::prelude::*;

use crate::{
    entity::{NormalFairyBulletEntity, NormalFairyEntity, PlayerBulletEntity, PlayerEntity},
    math::NormalizationVector2,
    window::Window,
};

pub fn update_render_player_bullet(world: &World, screen: &Window) {
    for (_, (_, position, _, _)) in &mut world.query::<PlayerBulletEntity>() {
        let pos = position
            .position
            .reset_from_vec2(*screen.playable_window().size())
            + (*screen.playable_window().get_start());
        draw_circle(pos.x, pos.y, 5.0, GRAY);
    }
}

pub fn update_render_enemy(world: &World, screen: &Window) {
    world
        .query::<NormalFairyEntity>()
        .iter()
        .for_each(|(_, (_, pos, _, _, _, _, sprite, _))| {
            sprite.draw(&pos, screen);
        });
}

pub fn update_render_normal_fairy_bullet(world: &World, screen: &Window) {
    world
        .query::<NormalFairyBulletEntity>()
        .iter()
        .for_each(|(_, (_, pos, _, _))| {
            let pos = pos
                .position
                .reset_from_vec2(*screen.playable_window().size())
                + (*screen.playable_window().get_start());
            draw_circle(pos.x, pos.y, 5.0, RED)
        });
}

pub fn update_render_player(world: &World, screen: &Window) {
    world
        .query::<PlayerEntity>()
        .iter()
        .for_each(|(_, (_, _, _, position, _, sprite))| {
            // INFO : Not working currently | causing the game to crash and burn
            // let material = shadow_shader_material(&screen).unwrap();
            // gl_use_material(&material);
            let _ = sprite.draw(&position, screen);
            // gl_use_default_material();
        });
}
