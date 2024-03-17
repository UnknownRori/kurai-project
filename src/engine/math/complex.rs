use std::f32::consts::FRAC_PI_2;

use macroquad::math::vec2;
use num_complex::Complex;

use super::{CartesianExt, ToVec2};

pub fn complx(re: f32, im: f32) -> Complex<f32> {
    Complex::new(re, im)
}

impl ToVec2 for Complex<f32> {
    fn to_vec2(&self) -> macroquad::prelude::Vec2 {
        vec2(self.re, self.im)
    }
}

pub trait ComplexExt: CartesianExt {
    fn distance_squared(&self, other: &Self);
    fn clamp(&self, min: &Self, max: &Self) -> Self;
    fn dir(&self, other: &Self) -> Self;
    fn normalize(&self) -> Self;
    fn rot(&self) -> f32;
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
    fn distance_squared(&self, other: &Self) {
        todo!()
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
