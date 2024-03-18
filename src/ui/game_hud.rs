use hecs::World;
use macroquad::{
    color::WHITE,
    math::vec2,
    text::{camera_font_scale, draw_text_ex, measure_text, Font, TextParams},
    time::get_fps,
};

use crate::{
    assets::konst::TEXTURE_HUD,
    components::hud::HUD,
    engine::{
        assets::AssetsManager,
        components::{CustomDraw, Layer2D, Sprite2D, Transform2D},
        math::complx,
    },
    konst::GAME_VERSION,
    score::ScoreData,
};

pub fn init_game_hud(assets_manager: &AssetsManager) -> (HUD, Sprite2D, Transform2D, Layer2D) {
    let texture = assets_manager
        .textures
        .get(TEXTURE_HUD)
        .expect("texture hud not found!");

    let sprite = Sprite2D::new(texture);
    let transform = Transform2D::new(complx(0.5, 0.5), vec2(1., 1.), 0.);

    (HUD, sprite, transform, Layer2D(100))
}

pub fn draw_hud_info(font: &Font) {
    let fps = format!("{}", get_fps());
    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(0.05);
    let len = measure_text(&fps, Some(font), font_size, font_scale);
    draw_text_ex(
        &fps,
        1. - len.width + 0.01,
        1.,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );

    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(0.03);
    let len = measure_text(GAME_VERSION, Some(font), font_size, font_scale);
    draw_text_ex(
        GAME_VERSION,
        1. - len.width + 0.03,
        len.height,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    )
}

pub fn draw_score(score: &ScoreData, font: &Font) {
    // TODO : Create immediate UI

    const SCALE: f32 = 0.05;

    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    let len = measure_text("Score", Some(font), font_size, font_scale);
    draw_text_ex(
        "Score",
        0.8 - len.width,
        0.1,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );

    let score_text = format!("{:08}", score.score);
    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    draw_text_ex(
        &score_text,
        0.8,
        0.1,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );

    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    let len = measure_text("Score", Some(font), font_size, font_scale);
    draw_text_ex(
        "Value",
        0.8 - len.width,
        0.12 + len.height,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );

    let value = format!("{:08}", score.value);
    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    draw_text_ex(
        &value,
        0.8,
        0.12 + len.height,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );

    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    let len = measure_text("Score", Some(font), font_size, font_scale);
    draw_text_ex(
        "Graze",
        0.8 - len.width,
        0.14 + len.height * 2.,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );

    let value = format!("{:04}", score.graze);
    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    draw_text_ex(
        &value,
        0.8,
        0.14 + len.height * 2.,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );

    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    let len = measure_text("Score", Some(font), font_size, font_scale);
    draw_text_ex(
        "Life",
        0.8 - len.width,
        0.34 + len.height,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );

    let value = format!("{}", score.life);
    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    draw_text_ex(
        &value,
        0.8,
        0.34 + len.height,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );
}

pub fn draw_entity_number(len: u32, font: &Font) {
    // INFO : For debugging purposes
    let total_entity = format!("{}", len);
    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(0.05);
    let len = measure_text(&total_entity, Some(font), font_size, font_scale);
    draw_text_ex(
        &total_entity,
        0.0,
        0.0 + len.height,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );
}
