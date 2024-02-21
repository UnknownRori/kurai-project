use macroquad::prelude::*;

use crate::assets::{AssetsHandler, AssetsManager};
use crate::constant::GAME_VERSION;
use crate::score::ScoreData;
use crate::window::Window;

pub struct StageUI {}

#[inline]
fn draw_scoreboard(window: &Window, score_data: &ScoreData, assets_manager: &AssetsManager) {
    draw_texture_ex(
        &assets_manager.textures.get("hud").unwrap(),
        window.game_window().get_start().x,
        window.game_window().get_start().y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(*window.game_window().size()),
            ..Default::default()
        },
    );

    draw_rectangle(
        window.playable_window().get_start().x + window.playable_window().size().x + 12.0,
        window.game_window().get_start().y + 42.0,
        window.game_window().size().x / 3.5,
        90.,
        Color::new(0., 0., 0., 0.5),
    );

    // TODO : Make this something more shorter, for loop or something
    draw_text(
        "Score",
        window.playable_window().get_start().x + window.playable_window().size().x + 12.0,
        window.game_window().get_start().y + 62.0,
        24.0,
        WHITE,
    );

    draw_text(
        format!("{:012}", score_data.score).as_str(),
        window.playable_window().get_start().x + window.playable_window().size().x + 72.0,
        window.game_window().get_start().y + 62.0,
        24.0,
        WHITE,
    );

    draw_text(
        "Value",
        window.playable_window().get_start().x + window.playable_window().size().x + 12.0,
        window.game_window().get_start().y + 94.0,
        24.0,
        WHITE,
    );

    draw_text(
        format!("{:08}", score_data.value).as_str(),
        window.playable_window().get_start().x + window.playable_window().size().x + 72.0,
        window.game_window().get_start().y + 94.0,
        24.0,
        WHITE,
    );

    draw_text(
        "Graze",
        window.playable_window().get_start().x + window.playable_window().size().x + 12.0,
        window.game_window().get_start().y + 124.0,
        24.0,
        WHITE,
    );

    draw_text(
        format!("{:04}", score_data.graze).as_str(),
        window.playable_window().get_start().x + window.playable_window().size().x + 72.0,
        window.game_window().get_start().y + 124.0,
        24.0,
        WHITE,
    );

    draw_text(
        "Life",
        window.playable_window().get_start().x + window.playable_window().size().x + 12.0,
        window.game_window().get_start().y + 184.0,
        24.0,
        WHITE,
    );

    draw_text(
        format!("{}", score_data.life).as_str(),
        window.playable_window().get_start().x + window.playable_window().size().x + 72.0,
        window.game_window().get_start().y + 184.0,
        24.0,
        WHITE,
    );
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

impl StageUI {
    pub async fn draw(window: &Window, score_data: &ScoreData, assets_manager: &AssetsManager) {
        draw_scoreboard(window, score_data, assets_manager);
    }
}
