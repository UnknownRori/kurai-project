use macroquad::{
    camera::{set_camera, set_default_camera, Camera2D},
    math::{uvec2, vec2, Rect, UVec2},
    miniquad::FilterMode,
    texture::{render_target, RenderTarget, Texture2D},
    window::{screen_height, screen_width},
};

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

#[derive(Debug, Default)]
pub struct ScreenBuffer2DBuilder {
    size: Option<UVec2>,
    filter: Option<FilterMode>,
}

impl ScreenBuffer2DBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn from_size(width: u32, height: u32) -> Self {
        Self {
            size: Some(uvec2(width, height)),
            filter: None,
        }
    }

    pub fn from_aspect_ratio(width: u32, aspect_ratio: f32) -> Self {
        Self {
            size: Some(uvec2(width, (width as f32 / aspect_ratio) as u32)),
            filter: None,
        }
    }

    pub fn filter(mut self, filter: FilterMode) -> Self {
        self.filter = Some(filter);
        self
    }

    pub fn size(mut self, size: UVec2) -> Self {
        self.size = Some(size);
        self
    }
}

impl From<ScreenBuffer2DBuilder> for ScreenBuffer2D {
    fn from(value: ScreenBuffer2DBuilder) -> Self {
        let filter = value.filter.unwrap_or(FilterMode::Nearest);
        let size = value
            .size
            .unwrap_or(uvec2(screen_width() as u32, screen_height() as u32));

        let render_target = render_target(size.x, size.y);
        render_target.texture.set_filter(filter);

        let camera = create_camera2d(Rect::new(0., 0., 1., 1.), render_target);

        Self { camera }
    }
}

#[derive(Debug, Default)]
pub struct ScreenBuffer2D {
    pub camera: Camera2D,
}

impl ScreenBuffer2D {
    pub fn new(size: UVec2, filter: FilterMode) -> Self {
        let rect = Rect::new(0., 0., 1., 1.);

        let render_target = render_target(size.x, size.y);
        render_target.texture.set_filter(filter);

        let camera = create_camera2d(rect, render_target);

        Self { camera }
    }

    pub fn texture(&self) -> &Texture2D {
        &self.camera.render_target.as_ref().unwrap().texture
    }

    pub fn set_camera(&self) {
        set_camera(&self.camera);
    }

    pub fn done_camera(&self) {
        set_default_camera();
    }
}
