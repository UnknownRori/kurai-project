mod draw;
mod update;

use hecs::World;
use macroquad::time::get_time;

use crate::{assets::AssetsManager, controls::Controls, score::ScoreData, window::Window};

use self::{
    draw::{
        draw_hitbox, update_render_enemy, update_render_normal_fairy_bullet, update_render_player,
        update_render_player_bullet,
    },
    update::{
        blink_death_entity, enemy_movement_update, enemy_shoot_normal_fairy, player_shoot,
        update_collision_detection_enemy_bullet_to_player,
        update_collision_detection_player_bullet_to_enemy, update_delete_bullet_offscreen,
        update_move_bullet, update_player_move,
    },
};

/// Register all the system related stuff of ECS here
pub fn update_system(
    world: &mut World,
    controls: &Controls,
    screen: &Window,
    delta: f32,
    time: f64,
    score: &mut ScoreData,
    assets_manager: &AssetsManager,
) {
    update_delete_bullet_offscreen(world, screen, delta, time);

    update_player_move(world, controls, screen, delta, time);
    update_move_bullet(world, screen, delta, time);
    player_shoot(world, assets_manager, controls, delta, time);
    enemy_shoot_normal_fairy(world, assets_manager, delta, time);
    enemy_movement_update(world, delta, time);
    update_collision_detection_enemy_bullet_to_player(world, assets_manager, score);
    update_collision_detection_player_bullet_to_enemy(world, score);

    blink_death_entity(world);
}

pub fn update_draw(world: &World, controls: &Controls, screen: &Window, time: f64, delta: f32) {
    update_render_player_bullet(world, screen);
    update_render_player(world, screen, controls, delta);
    update_render_enemy(world, screen);
    update_render_normal_fairy_bullet(world, screen);
    draw_hitbox(world, screen);
}
