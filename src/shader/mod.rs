use macroquad::prelude::*;

use crate::{
    assets::konst::{BLOOM_MATERIAL, LIGHTMAP},
    assets::AssetsManager,
    camera::screen_buffer2d::ScreenBuffer2D,
};

pub fn fetch_lightmap(assets: &AssetsManager, from: &ScreenBuffer2D, to: &ScreenBuffer2D) {
    let texture = from.texture();

    to.set_camera();
    clear_background(Color::new(0., 0., 0., 0.));

    let lightmap_shader = assets.shaders.get(LIGHTMAP).unwrap();
    lightmap_shader.set_uniform("iResolution", texture.size());
    gl_use_material(&lightmap_shader);
    draw_texture_ex(
        texture,
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(1., 1.)),
            ..Default::default()
        },
    );
    gl_use_default_material();
    to.done_camera()
}

pub fn generate_bloom(assets: &AssetsManager, from: &ScreenBuffer2D, to: &ScreenBuffer2D) {
    let texture = from.texture();

    to.set_camera();
    clear_background(Color::new(0., 0., 0., 0.));
    let bloom_shader = assets.shaders.get(BLOOM_MATERIAL).unwrap();
    bloom_shader.set_uniform("iResolution", texture.size());
    gl_use_material(&bloom_shader);
    bloom_shader.set_uniform("horizontal", 1);
    draw_texture_ex(
        texture,
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(1., 1.)),
            ..Default::default()
        },
    );
    bloom_shader.set_uniform("horizontal", 0);
    draw_texture_ex(
        texture,
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(1., 1.)),
            ..Default::default()
        },
    );
    gl_use_default_material();
    to.done_camera()
}
