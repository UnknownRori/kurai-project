use hecs::World;
use macroquad::prelude::*;

use crate::{
    components::{Hitbox, Position},
    controls::{Action, Controls},
    entity::{DrawableEnemyEntity, NormalFairyBulletEntity, PlayerBulletEntity, PlayerEntity},
    window::Window,
};

pub fn update_render_player_bullet(world: &World, screen: &Window) {
    for (_, (_, position, _, _, _, sprite)) in &mut world.query::<PlayerBulletEntity>() {
        sprite.draw(&position, screen);
    }
}

pub fn update_render_enemy(world: &World, screen: &Window) {
    world
        .query::<DrawableEnemyEntity>()
        .iter()
        .for_each(|(_, (_, pos, sprite))| {
            sprite.draw(&pos, screen);
        });
}

pub fn update_render_normal_fairy_bullet(world: &World, screen: &Window) {
    world
        .query::<NormalFairyBulletEntity>()
        .iter()
        .for_each(|(_, (_, pos, _, _, _, sprite))| {
            sprite.draw(pos, screen);
        });
}

pub fn draw_hitbox(world: &World, screen: &Window) {
    // world
    // .query::<(&Position, &Hitbox)>()
    // .iter()
    // .for_each(|(_, (position, hitbox))| {
    //     hitbox.draw(position, screen);
    // });
}

pub fn update_render_player(world: &World, screen: &Window, controls: &Controls, delta: f32) {
    world.query::<PlayerEntity>().iter().for_each(
        |(_, (player, _, _, position, _, sprite, hitbox, _, blink))| {
            if let Some(blink) = blink {
                blink.timer += 1.0 * delta;
                if blink.timer >= blink.speed_blink {
                    sprite.draw(&position, screen);
                    blink.timer = 0.;
                    blink.speed_blink /= blink.blink_decrease_ratio;
                }
            } else {
                sprite.draw(&position, screen);
            }

            if controls.is_down(&Action::Focus) {
                hitbox.draw(position, screen);
            }
        },
    );
}
