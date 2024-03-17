use hecs::World;

use crate::{controls::Action, engine::controls::Controls};

use self::{
    draw::{
        entity_draw::{game_entity_draw, player_focus_draw},
        hud_draw::hud_draw,
    },
    update::{
        ai_movement::enemy_movement_update, player_control::update_player_control,
        velocity::update_velocity,
    },
};

pub mod draw;
pub mod update;

pub fn update_system(world: &mut World, controls: &Controls<Action>, time: f64, delta: f32) {
    update_player_control(world, controls, time);
    enemy_movement_update(world, time, delta);
    update_velocity(world, delta);
}

pub fn update_draw(world: &World, controls: &Controls<Action>, time: f64, delta: f32) {
    game_entity_draw(world);
    player_focus_draw(world, controls, time);
}

pub fn update_draw_hud(world: &World, controls: &Controls<Action>, time: f64, delta: f32) {
    hud_draw(world);
}
