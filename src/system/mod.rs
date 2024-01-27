mod draw;
mod update;

use hecs::World;

use crate::{controls::Controls, window::Window};

use self::{
    draw::{
        update_render_enemy, update_render_normal_fairy_bullet, update_render_player,
        update_render_player_bullet,
    },
    update::{
        enemy_shoot_normal_fairy, player_shoot, update_delete_bullet_offscreen, update_move_bullet,
        update_player_move,
    },
};

/// Register all the system related stuff of ECS here
pub fn update_system(world: &mut World, controls: &Controls, screen: &Window) {
    update_delete_bullet_offscreen(world, screen);

    update_player_move(world, controls, screen);
    update_move_bullet(world, screen);
    player_shoot(world, controls);
    enemy_shoot_normal_fairy(world);
}

pub fn update_draw(world: &World, _controls: &Controls, screen: &Window) {
    update_render_player_bullet(world);
    update_render_player(world, screen);
    update_render_enemy(world);
    update_render_normal_fairy_bullet(world);
}
