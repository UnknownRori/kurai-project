use macroquad::math::Vec2;
use num_complex::Complex;

#[derive(Debug, Clone, Copy, Default)]
pub struct Transform2D {
    pub position: Complex<f32>,
    pub scale: Vec2,
    pub rotation: f32,
}

impl Transform2D {
    pub fn new(position: Complex<f32>, scale: Vec2, rotation: f32) -> Self {
        Self {
            position,
            scale,
            rotation,
        }
    }

    pub fn position(&self) -> &Complex<f32> {
        &self.position
    }

    pub fn scale(&self) -> &Vec2 {
        &self.scale
    }

    pub fn rotation(&self) -> &f32 {
        &self.rotation
    }
}

impl AsRef<Complex<f32>> for Transform2D {
    fn as_ref(&self) -> &Complex<f32> {
        self.position()
    }
}

impl AsRef<Vec2> for Transform2D {
    fn as_ref(&self) -> &Vec2 {
        self.scale()
    }
}

impl AsRef<f32> for Transform2D {
    fn as_ref(&self) -> &f32 {
        self.rotation()
    }
}
