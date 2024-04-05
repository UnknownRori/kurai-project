use macroquad::prelude::*;

pub fn rand_dir() -> Vec2 {
    vec2(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)).normalize()
}
