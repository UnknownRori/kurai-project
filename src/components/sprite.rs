use macroquad::prelude::*;

use super::transform2d::Transform2D;

#[derive(Debug, Clone)]
pub struct Sprite(Texture2D);

impl Sprite {
    pub fn new(texture: Texture2D) -> Self {
        Self(texture)
    }

    pub fn draw(&self, transform: &Transform2D, opacity: Option<f32>) {
        let half_scale = *transform.scale() / 2.;
        let color = Color::new(1., 1., 1., opacity.unwrap_or(1.));
        draw_texture_ex(
            &self.0,
            transform.position().re - half_scale.x,
            transform.position().im - half_scale.y,
            color,
            DrawTextureParams {
                rotation: -*transform.rotation(), // Not sure why it need to be negative
                dest_size: Some(*transform.scale()),
                ..Default::default()
            },
        );
    }
}
