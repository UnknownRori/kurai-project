use std::f32::consts::FRAC_PI_2;

use macroquad::math::vec2;
use num_complex::Complex;

use super::{CartesianExt, ToVec2};

impl ToVec2 for Complex<f32> {
    fn to_vec2(&self) -> macroquad::prelude::Vec2 {
        vec2(self.re, self.im)
    }
}

pub trait ComplexExt: CartesianExt {
    fn distance_squared(&self, other: &Self) -> f32;
    fn clamp(&self, min: &Self, max: &Self) -> Self;
    fn dir(&self, other: &Self) -> Self;
    fn cdir(angle: f32) -> Self;
    fn normalize(&self) -> Self;
    fn rot(&self) -> f32;
    fn lerp(&self, other: &Self, t: f32) -> Self;
}

impl<T> CartesianExt for Complex<T> {
    type Output = T;

    fn x(&self) -> &Self::Output {
        &self.re
    }

    fn y(&self) -> &Self::Output {
        &self.im
    }
}

impl ComplexExt for Complex<f32> {
    fn distance_squared(&self, other: &Self) -> f32 {
        (self - other).norm_sqr()
    }

    fn lerp(&self, other: &Self, t: f32) -> Self {
        t * (self - other) + self
    }

    fn cdir(angle: f32) -> Self {
        Complex::from_polar(1., angle)
    }

    fn clamp(&self, min: &Self, max: &Self) -> Self {
        Complex::new(self.re.clamp(min.re, max.re), self.im.clamp(min.im, max.im))
    }

    fn normalize(&self) -> Self {
        (self / self.norm()).clone()
    }

    fn dir(&self, other: &Self) -> Self {
        (other - self).normalize()
    }

    fn rot(&self) -> f32 {
        self.conj().arg() - FRAC_PI_2
    }
}
