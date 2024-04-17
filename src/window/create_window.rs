use macroquad::{
    math::IVec2,
    miniquad::conf::{Icon, Platform},
    window::Conf,
};

#[inline(always)]
pub fn create_window(
    window_title: String,
    fullscreen: bool,
    size: IVec2,
    window_resizable: bool,
    icon: Option<Icon>,
    sample_count: i32,
    high_dpi: bool,
    platform: Platform,
) -> Conf {
    Conf {
        window_title,
        fullscreen,
        window_width: size.x,
        window_height: size.y,
        window_resizable,
        icon,
        sample_count,
        high_dpi,
        platform,
    }
}
