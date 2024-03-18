mod complex;
mod normalization;
mod vec2;

pub use complex::{complx, ComplexExt};
use macroquad::math::Vec2;
pub use normalization::{NormalizationComplexf32, NormalizationVector2};
use num_complex::Complex;

pub trait CartesianExt {
    type Output;

    fn x(&self) -> &Self::Output;
    fn y(&self) -> &Self::Output;
}

pub trait ToVec2 {
    fn to_vec2(&self) -> Vec2;
}

pub trait ToComplex {
    fn to_cmpx(&self) -> Complex<f32>;
}
