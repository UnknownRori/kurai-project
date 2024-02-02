use macroquad::prelude::*;

use crate::assets::AssetsManager;
use crate::constant::{GAME_NAME, GAME_VERSION};
use crate::score::ScoreData;
use crate::window::Window;
use crate::{math::*, score};

pub struct StageUI {}

#[inline]
fn draw_scoreboard(window: &Window, score_data: &ScoreData, assets_manager: &AssetsManager) {
    draw_texture_ex(
        &assets_manager.get_texture("hud").unwrap(),
        window.game_window().get_start().x,
        window.game_window().get_start().y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(*window.game_window().size()),
            ..Default::default()
        },
    );

    // let score_pos = vec2(0.1, 0.2) * score_board + (*window.playable_window().get_end());
    // draw_text("Score", score_pos.x, 0.0 + 52.0, 32.0, WHITE);
    //
    // draw_text(
    //     format!("{:012}", score_data.score).as_str(),
    //     score_pos.x + (*window.game_window().get_start()).x,
    //     80.0,
    //     32.0,
    //     WHITE,
    // );

    draw_game_name(&window);
}

#[inline]
pub fn draw_version(window: &Window) {
    let len = measure_text(GAME_VERSION, None, 12, 1.0);
    draw_text(
        GAME_VERSION,
        window.game_window().get_end().x - len.width,
        window.game_window().get_start().y + 12.0,
        12.0,
        WHITE,
    );
}

#[inline]
pub fn draw_entity_number(window: &Window, number: u32) {
    let total = format!("{number}");
    draw_text(
        &total,
        window.game_window().get_end().x - 18.0,
        window.game_window().get_end().y - 32.0,
        12.0,
        WHITE,
    );
}
#[inline]
pub fn draw_fps(window: &Window, font_size: f32, color: Color) {
    let fps = format!("{}", get_fps());
    let len = measure_text(&fps, None, font_size as u16, 1.0);
    draw_text(
        &fps,
        window.game_window().get_end().x - len.width,
        window.game_window().get_end().y - 12.0,
        font_size,
        color,
    );
}

#[inline]
pub fn draw_game_name(window: &Window) {
    // draw_text(
    //     "DEMO!",
    //     window.get_playable_window().get_end_width()
    //         + (window.get_width() - window.get_playable_window().get_end_width()) / 2.5,
    //     window.get_height() - 50.0,
    //     32.0,
    //     WHITE,
    // );
    // draw_text(
    //     GAME_NAME,
    //     window.get_playable_window().get_end_width()
    //         + (window.get_width() - window.get_playable_window().get_end_width()) / 5.0,
    //     window.get_height() - 80.0,
    //     32.0,
    //     WHITE,
    // );
}

impl StageUI {
    pub async fn draw(window: &Window, score_data: &ScoreData, assets_manager: &AssetsManager) {
        draw_scoreboard(window, score_data, assets_manager);
    }
}
