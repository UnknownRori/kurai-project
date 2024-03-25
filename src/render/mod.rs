use hecs::World;
use macroquad::prelude::*;
use num_traits::ToPrimitive;

use crate::{
    controls::Action,
    engine::{assets::AssetsManager, camera::screen_buffer2d::ScreenBuffer2D, controls::Controls},
    scene::stage::StageManager,
    score::ScoreData,
    system::{update_draw, update_draw_hud},
    ui::game_hud::draw_entity_number,
};

pub fn draw_main_ui(
    world: &World,
    buffer: &ScreenBuffer2D,
    playable_buffer: &ScreenBuffer2D,
    controls: &Controls<Action>,
    font: &Font,
    score: &ScoreData,
    time: f64,
    delta: f32,
) {
    buffer.set_camera();
    clear_background(BLACK);

    let offset = vec2(0.03, 0.009);
    draw_texture_ex(
        playable_buffer.texture(),
        offset.x,
        offset.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(0.64, 0.975)),
            ..Default::default()
        },
    );

    update_draw_hud(world, controls, score, font, time, delta);
    draw_entity_number(world.len(), font);
    buffer.done_camera();
}

pub fn draw_stage(
    world: &World,
    assets_manager: &AssetsManager,
    stage_manager: &StageManager,
    buffer: &ScreenBuffer2D,
    controls: &Controls<Action>,
    time: f64,
    delta: f32,
) {
    buffer.set_camera();
    clear_background(BLACK);

    let material = assets_manager.shaders.get("stg1-bg").unwrap();
    material.set_uniform("iTime", time.to_f32().unwrap());
    material.set_uniform(
        "iResolution",
        vec2(buffer.texture().width(), buffer.texture().height()),
    );
    gl_use_material(&*material);
    draw_rectangle(0., 0., 1.0, 1.0, WHITE);
    gl_use_default_material();

    stage_manager.draw(time, delta);
    update_draw(&world, controls, time, delta);
    buffer.done_camera();
}
