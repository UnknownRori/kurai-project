mod assets;
mod attack_info;
mod components;
mod controls;
mod difficulty;
mod entity;
mod game;
mod konst;
mod math;
mod render;
mod scene;
mod score;
mod stage;
mod state;
mod system;
mod time;
mod ui;
mod utils;
mod window;

pub use game::Game;

use konst::GAME_NAME;
use macroquad::{prelude::ImageFormat, texture::Image, window::Conf};
use window::Icon;

pub fn kurai_window() -> Conf {
    let icon16 = Image::from_file_with_format(
        include_bytes!("../resources/icon/icon_16.png"),
        Some(ImageFormat::Png),
    )
    .unwrap();
    let icon32 = Image::from_file_with_format(
        include_bytes!("../resources/icon/icon_32.png"),
        Some(ImageFormat::Png),
    )
    .unwrap();
    let icon64 = Image::from_file_with_format(
        include_bytes!("../resources/icon/icon_64.png"),
        Some(ImageFormat::Png),
    )
    .unwrap();
    let icon = Icon::new(icon16, icon32, icon64);
    let icon = icon.icon();

    Conf {
        window_title: String::from(GAME_NAME),
        fullscreen: true,
        window_width: 900,
        window_height: 675,
        window_resizable: false,
        icon: Some(icon),
        sample_count: 1,
        high_dpi: true,
        platform: Default::default(),
    }
}
