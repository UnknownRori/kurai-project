use crate::constant::{
    DESIRED_ASPECT_RATIO, FULLSCREEN, GAME_NAME, HEIGHT, WIDTH, WINDOW_RESIZEABLE,
};
use macroquad::{miniquad::conf::Platform, prelude::*};

#[derive(Debug)]
pub struct PlayableWindow {
    start: Vec2,
    end: Vec2,
    size: Vec2,
}

/// Abstraction for dynamic window size
/// Make sure initialized once only, because what kind of idiot initialized this twice
#[derive(Debug)]
pub struct Window {
    screen: Vec2,
    aspect_ratio: f32,
    fullscreen: bool,
    game_window: PlayableWindow,
    playable_window: PlayableWindow,
}

impl PlayableWindow {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self {
            start,
            end,
            size: end - start,
        }
    }
}

impl Default for Window {
    fn default() -> Self {
        let (width, height, actual_aspect_ratio) = {
            (
                screen_width(),
                screen_height(),
                screen_width() / screen_height(),
            )
        };

        let adjusted = if actual_aspect_ratio > DESIRED_ASPECT_RATIO {
            vec2(height * DESIRED_ASPECT_RATIO, height)
        } else {
            vec2(width, width / DESIRED_ASPECT_RATIO)
        };

        let offset = vec2((width - adjusted.x) / 2f32, (height - adjusted.y) / 2f32);

        let game_window = PlayableWindow::new(
            vec2(offset.x, offset.y),
            vec2(offset.x + adjusted.x, offset.y + adjusted.y),
        );

        let padding = vec2(0.03, 0.009);
        let playable_pos = padding * (*game_window.size());
        let playable_window = PlayableWindow::new(
            vec2(
                game_window.get_start().x + playable_pos.x,
                game_window.get_start().y + playable_pos.y,
            ),
            vec2(
                game_window.get_end().x - playable_pos.x * 11f32,
                game_window.get_end().y - playable_pos.y * 1.3f32,
            ),
        );

        Self {
            screen: vec2(width, height),
            game_window,
            playable_window,
            fullscreen: FULLSCREEN,
            aspect_ratio: DESIRED_ASPECT_RATIO,
        }
    }
}

impl Window {
    pub fn update(&mut self) {
        // INFO : It's dumb but it work for time being
        *self = Self::default()
    }

    #[must_use]
    pub const fn screen(&self) -> &Vec2 {
        &self.screen
    }

    #[must_use]
    pub const fn game_window(&self) -> &PlayableWindow {
        &self.game_window
    }

    #[must_use]
    /// Area where the player and enemy are
    pub const fn playable_window(&self) -> &PlayableWindow {
        &self.playable_window
    }
}

impl PlayableWindow {
    #[must_use]
    pub const fn get_start(&self) -> &Vec2 {
        &self.start
    }
    #[must_use]
    pub const fn get_end(&self) -> &Vec2 {
        &self.end
    }

    #[must_use]
    pub const fn size(&self) -> &Vec2 {
        &self.size
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
        window_resizable: WINDOW_RESIZEABLE,
        icon: None, // TODO : Update this later
        high_dpi: true,
        platform: Platform::default(),
        ..Default::default()
    }
}
