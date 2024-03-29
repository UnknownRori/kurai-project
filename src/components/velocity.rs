use keyframe::{keyframes, AnimationSequence};
use num_complex::Complex;

use crate::engine::{
    math::{complx, ComplexExt},
    time::Instant,
};

/// Velocity type that will update the Transform2D
pub enum Velocity {
    Normal(Complex<f32>),
    Multiply(Complex<f32>),
}

impl Velocity {
    pub fn set(&mut self, speed: Complex<f32>) {
        match self {
            Velocity::Normal(a) => *a = speed,
            Velocity::Multiply(a) => *a = speed,
        }
    }

    pub fn get(&self) -> &Complex<f32> {
        match self {
            Velocity::Normal(a) => a,
            Velocity::Multiply(a) => a,
        }
    }
}

impl AsRef<Complex<f32>> for Velocity {
    fn as_ref(&self) -> &Complex<f32> {
        match self {
            Velocity::Normal(a) => a,
            Velocity::Multiply(a) => a,
        }
    }
}

impl AsMut<Complex<f32>> for Velocity {
    fn as_mut(&mut self) -> &mut Complex<f32> {
        match self {
            Velocity::Normal(a) => a,
            Velocity::Multiply(a) => a,
        }
    }
}

/// Damping value for modifying Velocity enum overtime
pub struct DampedVelocity(pub f32);

/// Acceleration that will modify Velocity enum overtime
pub struct AcceleratedVelocity {
    // Speed Cap
    pub max_speed: f32,

    // Starting acceleration before default acceleration
    pub starting_acceleration: f32,

    // Acceleration overtime
    pub acceleration: f32,

    pub acceleration_keyframe: (AnimationSequence<f32>, AnimationSequence<f32>),

    pub timer: (Option<Instant>, Option<Instant>),
}

impl AcceleratedVelocity {
    pub fn new(
        max_speed: f32,
        starting_acceleration: f32,
        acceleration: f32,
        acceleration_time: f32,
        ease: impl keyframe::EasingFunction + Sync + Send + Copy + 'static,
    ) -> Self {
        Self {
            max_speed,
            starting_acceleration,
            acceleration,
            timer: (None, None),
            acceleration_keyframe: (
                keyframes![(0., 0., ease), (1., acceleration_time, ease)],
                keyframes![(0., 0., ease), (1., acceleration_time, ease)],
            ),
        }
    }

    /// Update the current_velocity with the current speed
    /// add speed should be normalized from -1 to 1
    pub fn update(
        &mut self,
        add: Complex<f32>,
        current_velocity: Complex<f32>,
        delta: f32,
        time: f64,
        // max_speed: Option<f32>,
    ) -> Complex<f32> {
        // TODO : Refactor this later
        let mut new_vel = {
            let re = if self.acceleration_keyframe.0.progress() == 0. {
                add.re * self.starting_acceleration
            } else {
                add.re
            };

            let im = if self.acceleration_keyframe.1.progress() == 0. {
                add.im * self.starting_acceleration
            } else {
                add.im
            };

            complx(
                if im != 0. { re / 2. } else { re },
                if re != 0. { im / 2. } else { im },
            )
        };

        if add.re != 0. {
            self.acceleration_keyframe.0.advance_by(time);
        } else if add.re == 0. {
            self.acceleration_keyframe.0.advance_to(0.);
        }

        if add.im != 0. {
            self.acceleration_keyframe.1.advance_by(time);
        } else if add.im == 0. {
            self.acceleration_keyframe.1.advance_to(0.);
        }

        new_vel = complx(
            new_vel.re * (self.acceleration * self.acceleration_keyframe.0.now()),
            new_vel.im * (self.acceleration * self.acceleration_keyframe.1.now()),
        );
        new_vel += current_velocity;

        new_vel.clamp(
            &complx(-self.max_speed, -self.max_speed),
            &complx(self.max_speed, self.max_speed),
        )
    }
}

/// Constant Acceleration one direction that will modify Velocity enum overtime
/// It will apply force overtime
/// INFO : Still experimenting
pub struct ConstantAcceleration {
    // Speed Cap
    pub max_speed: (Complex<f32>, Complex<f32>),

    // Starting acceleration before default acceleration
    pub starting_acceleration: Complex<f32>,

    // Acceleration overtime
    pub acceleration: f32,

    pub acceleration_keyframe: (AnimationSequence<f32>, AnimationSequence<f32>),

    pub timer: Instant,
}

impl ConstantAcceleration {
    pub fn new(
        max_speed: f32,
        starting_acceleration: Complex<f32>,
        acceleration: f32,
        acceleration_time: f32,
        ease: impl keyframe::EasingFunction + Sync + Send + Copy + 'static,
    ) -> Self {
        Self {
            max_speed: (
                starting_acceleration * max_speed,
                -(starting_acceleration * max_speed),
            ),
            starting_acceleration,
            acceleration,
            timer: Instant::now(),
            acceleration_keyframe: (
                keyframes![(0., 0., ease), (1., acceleration_time, ease)],
                keyframes![(0., 0., ease), (1., acceleration_time, ease)],
            ),
        }
    }

    /// Update the current_velocity with the current speed
    /// add speed should be normalized from -1 to 1
    pub fn update(
        &mut self,
        current_velocity: Complex<f32>,
        delta: f32,
        time: f64,
        // max_speed: Option<f32>,
    ) -> Complex<f32> {
        self.acceleration_keyframe
            .0
            .advance_by(self.timer.elapsed(time));

        let new_vel =
            self.starting_acceleration * (self.acceleration * self.acceleration_keyframe.0.now());
        new_vel
    }
}
