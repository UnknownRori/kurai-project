use std::sync::Arc;

use macroquad::{
    color::WHITE,
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

use super::Transform2D;

#[derive(Debug, Clone)]
pub struct Sprite2D(Arc<Texture2D>);

impl Sprite2D {
    pub fn new(texture: Arc<Texture2D>) -> Self {
        Self(texture)
    }

    pub fn draw(&self, transform: &Transform2D) {
        let half_scale = *transform.scale() / 2.;
        draw_texture_ex(
            &*self.0,
            transform.position().re - half_scale.x,
            transform.position().im - half_scale.y,
            WHITE,
            DrawTextureParams {
                rotation: *transform.rotation(),
                dest_size: Some(*transform.scale()),
                ..Default::default()
            },
        );
    }
}
