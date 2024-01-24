mod draw;
mod update;

use hecs::World;

use crate::{controls::Controls, window::Window};

use self::{
    draw::{update_render_player, update_render_player_bullet},
    update::{player_shoot, update_move_bullet, update_player_move},
};

/// Register all the system related stuff of ECS here
pub fn update_system(world: &mut World, controls: &Controls, screen: &Window) {
    update_player_move(world, controls, screen);
    update_move_bullet(world, screen);
    player_shoot(world, controls);
}

pub fn update_draw(world: &World, _controls: &Controls, _screen: &Window) {
    update_render_player_bullet(world);
    update_render_player(world);
}
