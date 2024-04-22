use macroquad::prelude::*;

pub fn create_camera2d(rect: Rect, render_target: RenderTarget) -> Camera2D {
    let mut camera = Camera2D::from_display_rect(rect);
    camera.zoom = vec2(1. / rect.w * 2., 1. / rect.h * 2.);
    camera.render_target = Some(render_target);
    camera
        .render_target
        .as_ref()
        .unwrap()
        .texture
        .set_filter(FilterMode::Nearest);

    camera
}
