use crate::constant::{ASPECT_RATIO, FULLSCREEN, GAME_NAME, HEIGHT, WIDTH};
use macroquad::{miniquad::conf::Platform, prelude::*};

pub struct PlayableWindow {
    start_width: f32,
    start_height: f32,
    end_width: f32,
    end_height: f32,
}

/// Abstraction for dynamic window size
/// Make sure initialized once only, because what kind of idiot initialized this twice
pub struct Window {
    height: f32,
    width: f32,
    aspect_ratio: f32,
    fullscreen: bool,

    playable_window: PlayableWindow,
}

impl Default for Window {
    fn default() -> Self {
        let (width, height, aspect_ratio) = if FULLSCREEN {
            (
                screen_width(),
                screen_height(),
                screen_width() / screen_height(),
            )
        } else {
            (WIDTH, HEIGHT, ASPECT_RATIO)
        };

        Self {
            width,
            height,
            playable_window: PlayableWindow {
                start_width: 0.0,
                start_height: 0.0,
                end_width: width - width / 2.5,
                end_height: height,
            },
            fullscreen: FULLSCREEN,
            aspect_ratio,
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

    #[must_use]
    pub const fn get_playable_window(&self) -> &PlayableWindow {
        &self.playable_window
    }
}

impl PlayableWindow {
    #[must_use]
    pub const fn get_start_width(&self) -> &f32 {
        &self.start_width
    }
    #[must_use]
    pub const fn get_end_width(&self) -> &f32 {
        &self.end_width
    }
    #[must_use]
    pub const fn get_start_height(&self) -> &f32 {
        &self.start_height
    }
    #[must_use]
    pub const fn get_end_height(&self) -> &f32 {
        &self.end_height
    }
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn window_conf() -> Conf {
    Conf {
        window_title: GAME_NAME.to_owned(),
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
