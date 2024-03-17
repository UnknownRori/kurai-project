use num_complex::Complex;

pub enum Velocity {
    Normal(Complex<f32>),
    Multiply(Complex<f32>),
}
