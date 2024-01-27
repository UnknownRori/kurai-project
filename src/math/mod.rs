use num_complex::Complex;
use num_traits::{Float, MulAdd};

pub trait CartesianCoordinate<T> {
    fn x(&self) -> &T;
    fn y(&self) -> &T;
}

impl<T> CartesianCoordinate<T> for Complex<T> {
    fn x(&self) -> &T {
        &self.re
    }

    fn y(&self) -> &T {
        &self.im
    }
}

pub trait ExtendedComplexNumber {
    #[must_use]
    fn normalize(&self) -> Self;

    #[must_use]
    fn lerp(&self, other: Complex<f32>, t: f32) -> Self;
}

/// Extending the available method for Complex<f32> Number
impl ExtendedComplexNumber for Complex<f32> {
    fn normalize(&self) -> Self {
        (self / self.norm()).clone()
    }

    fn lerp(&self, other: Complex<f32>, t: f32) -> Self {
        let re = (1.0 - t).mul_add(self.re, t * other.re);
        let im = (1.0 - t).mul_add(self.im, t * other.im);

        Self::new(re, im)
    }
}
