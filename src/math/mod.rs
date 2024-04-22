mod complex;
mod vec2;

pub use complex::ComplexExt;
use macroquad::math::Vec2;
use num_complex::Complex;

#[macro_export]
macro_rules! cmpx {
    ($x:expr) => {
        num_complex::Complex::new($x, $x)
    };
    ($re:expr, $im:expr) => {
        num_complex::Complex::new($re, $im)
    };
    (($re:expr, $im:expr)) => {
        num_complex::Complex::new($re, $im)
    };
}

#[macro_export]
macro_rules! vec2 {
    ($x:expr) => {
        macroquad::math::Vec2::new($x, $x)
    };
    ($x:expr, $y:expr) => {
        macroquad::math::Vec2::new($x, $y)
    };
}

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
