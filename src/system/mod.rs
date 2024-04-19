use hecs::World;
use macroquad::text::Font;

use crate::{
    assets::AssetsManager,
    controls::{Action, Controls},
    score::ScoreData,
    ui::game_hud::{draw_hud_info, draw_score},
    utils::FPSCounter,
};

use self::{
    draw::{
        collision_draw::collision_draw,
        entity_draw::{game_entity_draw, player_focus_draw},
    },
    update::{
        ai_movement::enemy_movement_update, attack_info_trigger::attack_info_trigger,
        collision::collision_player_with_enemy_bullets, delete_bullet_offmap::delete_bullet_offmap,
        movement::update_movement, player_control::update_player_control,
    },
};

pub mod draw;
pub mod update;

pub fn update_system(
    world: &mut World,
    controls: &Controls<Action>,
    score: &mut ScoreData,
    time: f64,
    delta: f32,
) {
    update_player_control(world, controls, delta, time);
    enemy_movement_update(world, time, delta);
    attack_info_trigger(world, time, delta);
    update_movement(world, delta);

    collision_player_with_enemy_bullets(world, score);
    delete_bullet_offmap(world);
}

pub fn update_draw(
    world: &World,
    controls: &Controls<Action>,
    assets: &AssetsManager,
    time: f64,
    _delta: f32,
) {
    game_entity_draw(world);
    player_focus_draw(world, controls, time);
    collision_draw(world);
}

pub fn update_draw_hud(
    world: &World,
    _controls: &Controls<Action>,
    score: &ScoreData,
    fps_counter: &FPSCounter,
    font: &Font,
    _time: f64,
    _delta: f32,
) {
    draw_hud_info(font, fps_counter);
    draw_score(score, font);
}
