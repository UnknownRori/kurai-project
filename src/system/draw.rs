use hecs::World;
use macroquad::prelude::*;
use num_traits::ToPrimitive;

use crate::{
    assets::{AssetsHandler, AssetsManager},
    components::{Hitbox, Position},
    controls::{Action, Controls},
    entity::{DrawableEnemyEntity, NormalFairyBulletEntity, PlayerBulletEntity, PlayerEntity},
    math::{NormalizationVector2, ToVec2},
    window::Window,
};

pub fn update_render_player_bullet(world: &World, screen: &Window) {
    for (_, (_, position, _, _, _, sprite)) in &mut world.query::<PlayerBulletEntity>() {
        sprite.draw(&position, screen, 1.);
    }
}

pub fn update_render_enemy(world: &World, screen: &Window) {
    world
        .query::<DrawableEnemyEntity>()
        .iter()
        .for_each(|(_, (_, pos, sprite))| {
            sprite.draw(&pos, screen, 1.);
        });
}

pub fn update_render_normal_fairy_bullet(world: &World, screen: &Window) {
    world
        .query::<NormalFairyBulletEntity>()
        .iter()
        .for_each(|(_, (_, pos, _, _, _, sprite))| {
            sprite.draw(pos, screen, 1.);
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

pub fn update_render_player(
    world: &World,
    assets: &AssetsManager,
    screen: &Window,
    controls: &Controls,
    delta: f32,
    time: f64,
) {
    world.query::<PlayerEntity>().iter().for_each(
        |(_, (player, _, _, position, _, sprite, hitbox, _, blink))| {
            if let Some(blink) = blink {
                blink.timer += 1.0 * delta;
                if blink.timer >= blink.speed_blink {
                    sprite.draw(&position, screen, 1.);
                    blink.timer = 0.;
                    blink.speed_blink /= blink.blink_decrease_ratio;
                } else {
                    sprite.draw(&position, screen, 0.2);
                }
            } else {
                sprite.draw(&position, screen, 1.);
            }

            if controls.is_down(&Action::Focus) {
                // TODO : Still thinking should i put this on to components (?)
                let texture = assets.textures.get("focus").unwrap();
                let size = vec2(0.12, 0.12);
                let pos = position.position.to_vec2() - size / 2.0;
                let real_pos = pos.reset_from_vec2(*screen.playable_window().size())
                    + *screen.playable_window().get_start();
                let real_size = size.reset_from_vec2(*screen.playable_window().size());
                draw_texture_ex(
                    &*texture,
                    real_pos.x,
                    real_pos.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(real_size),
                        ..Default::default()
                    },
                );

                draw_texture_ex(
                    &*texture,
                    real_pos.x,
                    real_pos.y,
                    WHITE,
                    DrawTextureParams {
                        rotation: time.to_f32().unwrap_or(0.) % 360.0 * 2.,
                        dest_size: Some(real_size),
                        ..Default::default()
                    },
                );
            }
        },
    );
}
