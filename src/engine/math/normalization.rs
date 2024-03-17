use macroquad::math::{vec2, Vec2};
use num_complex::Complex;

pub trait NormalizationVector2 {
    /// Normalize the value against Vec2
    #[must_use]
    fn normalize_from_vec2(&self, target: Vec2) -> Vec2;

    /// Resize the value based on the target_vec using normalized value
    #[must_use]
    fn resize_vec2(&self, target_normalize: Vec2, target_size: Vec2) -> Vec2;

    /// Undo the normalization
    #[must_use]
    fn reset_from_vec2(&self, target_reset: Vec2) -> Vec2;
}

pub trait NormalizationComplexf32 {
    /// Normalize the value against Vec2
    fn normalize_from_compx(&self, target: Complex<f32>) -> Complex<f32>;
    fn resize_compx(
        &self,
        target_normalize: Complex<f32>,
        target_size: Complex<f32>,
    ) -> Complex<f32>;

    /// Undo the normalization
    #[must_use]
    fn reset_from_compx(&self, target_reset: Complex<f32>) -> Complex<f32>;
}

impl NormalizationVector2 for Vec2 {
    #[inline]
    fn normalize_from_vec2(&self, target: Vec2) -> Vec2 {
        vec2(self.x / target.x, self.y / target.y)
    }

    #[inline]
    fn resize_vec2(&self, target_normalize: Vec2, target_size: Vec2) -> Vec2 {
        let normalized = self.normalize_from_vec2(target_normalize);
        let aspect_ratio = normalized.x / normalized.y;

        vec2(target_size.x, target_size.y / aspect_ratio)
    }

    fn reset_from_vec2(&self, target_reset: Vec2) -> Vec2 {
        *self * target_reset
    }
}

impl NormalizationVector2 for Complex<f32> {
    #[inline]
    fn normalize_from_vec2(&self, target: Vec2) -> Vec2 {
        vec2(self.re / target.x, self.im / target.y)
    }

    #[inline]
    fn resize_vec2(&self, target_normalize: Vec2, target_size: Vec2) -> Vec2 {
        let normalized = self.normalize_from_vec2(target_normalize);
        let aspect_ratio = normalized.x / normalized.y;

        vec2(target_size.x, target_size.y / aspect_ratio)
    }

    fn reset_from_vec2(&self, target_reset: Vec2) -> Vec2 {
        vec2(self.re * target_reset.x, self.im * target_reset.y)
    }
}

impl NormalizationComplexf32 for Vec2 {
    #[inline]
    fn normalize_from_compx(&self, target: Complex<f32>) -> Complex<f32> {
        Complex::new(self.x / target.re, self.y / target.im)
    }

    #[inline]
    fn resize_compx(
        &self,
        target_normalize: Complex<f32>,
        target_size: Complex<f32>,
    ) -> Complex<f32> {
        let normalized = self.normalize_from_compx(target_normalize);
        let aspect_ratio = normalized.re / normalized.im;

        Complex::new(target_size.re, target_size.im / aspect_ratio)
    }

    fn reset_from_compx(&self, target_reset: Complex<f32>) -> Complex<f32> {
        Complex::new(self.x * target_reset.re, self.y * target_reset.im)
    }
}

impl NormalizationComplexf32 for Complex<f32> {
    #[inline]
    fn normalize_from_compx(&self, target: Complex<f32>) -> Complex<f32> {
        Complex::new(self.re / target.re, self.im / target.im)
    }

    #[inline]
    fn resize_compx(
        &self,
        target_normalize: Complex<f32>,
        target_size: Complex<f32>,
    ) -> Complex<f32> {
        let normalized = self.normalize_from_compx(target_normalize);
        let aspect_ratio = normalized.re / normalized.im;

        Complex::new(target_size.re, target_size.im / aspect_ratio)
    }

    fn reset_from_compx(&self, target_reset: Complex<f32>) -> Complex<f32> {
        Complex::new(self.re / target_reset.re, self.im / target_reset.im)
    }
}
