use crate::constant::{ASPECT_RATIO, FULLSCREEN, HEIGHT, WIDTH};
use macroquad::{miniquad::conf::Platform, prelude::*};

/// Abstraction for dynamic window size
/// Make sure initialized once only, because what kind of idiot initialized this twice
pub struct Window {
    height: f32,
    width: f32,
    aspect_ratio: f32,
    fullscreen: bool,
}

impl Default for Window {
    fn default() -> Self {
        if FULLSCREEN {
            Self {
                width: screen_width(),
                height: screen_height(),
                aspect_ratio: screen_width() / screen_height(),
                fullscreen: true,
            }
        } else {
            Self {
                width: WIDTH,
                height: HEIGHT,
                aspect_ratio: ASPECT_RATIO,
                fullscreen: false,
            }
        }
    }
}

impl Window {
    #[must_use]
    pub const fn get_width(&self) -> &f32 {
        &self.width
    }

    #[must_use]
    pub const fn get_height(&self) -> &f32 {
        &self.height
    }
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn window_conf() -> Conf {
    Conf {
        window_title: "Scarlet Project".to_owned(),
        fullscreen: FULLSCREEN,
        window_height: HEIGHT.floor() as i32,
        window_width: WIDTH.floor() as i32,
        window_resizable: false,
        icon: None, // TODO : Update this later
        high_dpi: true,
        platform: Platform::default(),
        ..Default::default()
    }
}
