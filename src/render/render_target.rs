use macroquad::texture::{render_target, RenderTarget};

pub fn create_render_target_from_aspect_ratio(width: u32, ratio: f32) -> RenderTarget {
    let height = width as f32 / ratio;

    render_target(width, height as u32)
}
