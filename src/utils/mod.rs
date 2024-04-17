use macroquad::prelude::*;

mod fps_counter;

pub use fps_counter::FPSCounter;

pub fn rand_dir() -> Vec2 {
    vec2(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)).normalize()
}
