mod draw;
mod update;

use hecs::World;
use macroquad::time::get_time;

use crate::{controls::Controls, window::Window};

use self::{
    draw::{
        update_render_enemy, update_render_normal_fairy_bullet, update_render_player,
        update_render_player_bullet,
    },
    update::{
        enemy_movement_update, enemy_shoot_normal_fairy, player_shoot,
        update_delete_bullet_offscreen, update_move_bullet, update_player_move,
    },
};

/// Register all the system related stuff of ECS here
pub fn update_system(
    world: &mut World,
    controls: &Controls,
    screen: &Window,
    delta: f32,
    time: f64,
) {
    update_delete_bullet_offscreen(world, screen, delta, time);

    update_player_move(world, controls, screen, delta, time);
    update_move_bullet(world, screen, delta, time);
    player_shoot(world, controls, delta, time);
    enemy_shoot_normal_fairy(world, delta, time);
    enemy_movement_update(world, delta, time);
}

pub fn update_draw(world: &World, _controls: &Controls, screen: &Window) {
    update_render_player_bullet(world, screen);
    update_render_player(world, screen);
    update_render_enemy(world, screen);
    update_render_normal_fairy_bullet(world, screen);
}
