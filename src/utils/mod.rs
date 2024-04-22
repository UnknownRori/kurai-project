pub use macroquad::prelude::*;

pub struct FPSCounter {
    smoothing_factor: f32,
    refresh_freq: f32,
    time_since_last_update: f32,
    average_fps: f32,
    fps: f32,
}

impl Default for FPSCounter {
    fn default() -> Self {
        Self {
            smoothing_factor: 0.9,
            refresh_freq: 0.1,
            time_since_last_update: 0.,
            average_fps: 0.,
            fps: 0.,
        }
    }
}

impl FPSCounter {
    pub fn update(&mut self) {
        self.average_fps = self.smoothing_factor * self.average_fps
            + (1. - self.smoothing_factor) * 1. / get_frame_time();

        if self.time_since_last_update <= self.refresh_freq {
            self.time_since_last_update += get_frame_time();
            return;
        }

        self.fps = self.average_fps;

        self.time_since_last_update = 0.;
    }
    pub fn draw(&self, font: &Font) {
        let fps = format!("{}", self.fps.round());
        let (font_size, font_scale, font_scale_aspect) = camera_font_scale(0.025);
        let len = measure_text(&fps, Some(font), font_size, font_scale);
        draw_text_ex(
            &fps,
            1. - len.width + 0.01,
            1.,
            TextParams {
                color: WHITE,
                font: Some(font),
                font_size,
                font_scale,
                font_scale_aspect,
                ..Default::default()
            },
        );
    }
}

pub fn rand_dir() -> Vec2 {
    vec2(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)).normalize()
}

#[inline(always)]
pub fn get_adjusted_screen(aspect_ratio: f32) -> Vec2 {
    let (width, height, actual_aspect_ratio) = {
        (
            screen_width(),
            screen_height(),
            screen_width() / screen_height(),
        )
    };

    let adjusted = if (actual_aspect_ratio * 10.).round() > (aspect_ratio * 10.).round() {
        vec2(height * aspect_ratio, height)
    } else {
        vec2(width, width / aspect_ratio)
    };

    adjusted
}
