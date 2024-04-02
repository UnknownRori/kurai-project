use macroquad::prelude::*;

use super::Transform2D;

#[derive(Debug, Clone)]
pub struct Sprite2D(Texture2D);

impl Sprite2D {
    pub fn new(texture: Texture2D) -> Self {
        Self(texture)
    }

    pub fn draw(&self, transform: &Transform2D) {
        let half_scale = *transform.scale() / 2.;
        draw_texture_ex(
            &self.0,
            transform.position().re - half_scale.x,
            transform.position().im - half_scale.y,
            WHITE,
            DrawTextureParams {
                rotation: -*transform.rotation(), // Not sure why it need to be negative
                dest_size: Some(*transform.scale()),
                ..Default::default()
            },
        );
    }
}
