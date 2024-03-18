use macroquad::{math::ivec2, miniquad::conf::Platform, window::Conf};

use crate::{
    engine::window::create_window,
    konst::{FULLSCREEN, GAME_NAME, WINDOW_RESIZEABLE},
};

pub fn game_window() -> Conf {
    create_window(
        GAME_NAME.to_string(),
        FULLSCREEN,
        ivec2(900, 675),
        WINDOW_RESIZEABLE,
        None,
        1,
        true,
        Platform::default(),
    )
}
