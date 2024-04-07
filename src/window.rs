use macroquad::{miniquad::conf::Platform, prelude::*, window::Conf};

use crate::{
    engine::{ui::icon::KuraiIcon, window::create_window},
    konst::{FULLSCREEN, GAME_NAME, WINDOW_RESIZEABLE},
};

pub fn game_window() -> Conf {
    let kurai_icon_16 = Image::from_file_with_format(
        include_bytes!("../resources/icon/icon_16.png"),
        Some(ImageFormat::Png),
    )
    .unwrap();
    let kurai_icon_32 = Image::from_file_with_format(
        include_bytes!("../resources/icon/icon_32.png"),
        Some(ImageFormat::Png),
    )
    .unwrap();
    let kurai_icon_64 = Image::from_file_with_format(
        include_bytes!("../resources/icon/icon_64.png"),
        Some(ImageFormat::Png),
    )
    .unwrap();
    let icon = KuraiIcon::new(kurai_icon_16, kurai_icon_32, kurai_icon_64);
    let icon = icon.icon();

    create_window(
        GAME_NAME.to_string(),
        FULLSCREEN,
        ivec2(900, 675),
        WINDOW_RESIZEABLE,
        Some(icon),
        1,
        true,
        Platform::default(),
    )
}
