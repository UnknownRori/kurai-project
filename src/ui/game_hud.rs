use macroquad::{
    color::WHITE,
    text::{camera_font_scale, draw_text_ex, measure_text, Font, TextParams},
};

use crate::{konst::GAME_VERSION, score::ScoreData, utils::FPSCounter};

pub fn draw_hud_info(font: &Font, fps_counter: &FPSCounter) {
    fps_counter.draw(font);

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

    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    let len = measure_text("Score", Some(font), font_size, font_scale);
    draw_text_ex(
        "Spell",
        0.8 - len.width,
        0.38 + len.height,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );

    let value = format!("{}", score.spell);
    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    draw_text_ex(
        &value,
        0.8,
        0.38 + len.height,
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
