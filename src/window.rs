use crate::constant::{HEIGHT, WIDTH};
use macroquad::prelude::*;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Scarlet Project".to_owned(),
        fullscreen: false,
        window_height: HEIGHT,
        window_width: WIDTH,
        window_resizable: false,
        icon: None, // TODO : Update this later
        high_dpi: true,
        platform: miniquad::conf::Platform {
            ..Default::default()
        },
        ..Default::default()
    }
}
