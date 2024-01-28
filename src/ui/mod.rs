use macroquad::prelude::*;

use crate::assets::AssetsManager;
use crate::constant::{GAME_NAME, GAME_VERSION};
use crate::score::ScoreData;
use crate::window::Window;

pub struct StageUI {}

#[inline]
fn draw_scoreboard(window: &Window, score_board: &ScoreData, assets_manager: &AssetsManager) {
    let width_score = window.get_width() / 2.5;
    draw_texture_ex(
        &assets_manager.get_texture("stage_ui").unwrap(),
        window.get_width() - width_score,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(width_score, *window.get_height())),
            ..Default::default()
        },
    );
    // draw_rectangle(
    //     window.get_width() - width_score,
    //     0.0,
    //     width_score,
    //     *window.get_height(),
    //     RED,
    // );

    draw_text(
        "Score",
        window.get_playable_window().get_end_width()
            + (window.get_width() - window.get_playable_window().get_end_width()) / 5.0,
        50.0,
        32.0,
        WHITE,
    );

    draw_text(
        format!("{:012}", score_board.score).as_str(),
        window.get_playable_window().get_end_width()
            + (window.get_width() - window.get_playable_window().get_end_width()) / 5.0,
        80.0,
        32.0,
        WHITE,
    );

    draw_game_name(&window);
}

#[inline]
pub fn draw_version(window: &Window) {
    draw_text(
        GAME_VERSION,
        window.get_width() - <usize as num_traits::AsPrimitive<f32>>::as_(GAME_VERSION.len() * 6),
        12.0,
        12.0,
        WHITE,
    );
}

#[inline]
pub fn draw_entity_number(window: &Window, number: u32) {
    draw_text(
        format!("{number}").as_str(),
        window.get_width() - 18.0,
        window.get_height() - 32.0,
        12.0,
        WHITE,
    );
}

#[inline]
pub fn draw_fps(window: &Window, font_size: f32, color: Color) {
    draw_text(
        format!("{}", get_fps()).as_str(),
        window.get_width() - 32.0,
        window.get_height() - 4.0,
        font_size,
        color,
    );
}

#[inline]
pub fn draw_game_name(window: &Window) {
    draw_text(
        "DEMO!",
        window.get_playable_window().get_end_width()
            + (window.get_width() - window.get_playable_window().get_end_width()) / 2.5,
        window.get_height() - 50.0,
        32.0,
        WHITE,
    );
    draw_text(
        GAME_NAME,
        window.get_playable_window().get_end_width()
            + (window.get_width() - window.get_playable_window().get_end_width()) / 5.0,
        window.get_height() - 80.0,
        32.0,
        WHITE,
    );
}

impl StageUI {
    pub async fn draw(window: &Window, score_data: &ScoreData, assets_manager: &AssetsManager) {
        draw_scoreboard(window, score_data, assets_manager);
    }
}
