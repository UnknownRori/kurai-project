use num_complex::Complex;

#[derive(Debug, Clone, Copy, Default)]
pub struct MoveParams {
    pub velocity: Complex<f32>,
    pub acceleration: Complex<f32>,
    pub retention: f32,
    pub attraction: Complex<f32>,
    pub attraction_point: Complex<f32>,
    pub attraction_exponent: f32,
}
impl MoveParams {
    pub fn update(&mut self, pos: &mut Complex<f32>, delta: f32) -> Complex<f32> {
        let orig_velocity = self.velocity;
        *pos += orig_velocity * delta;

        self.velocity = self.acceleration * delta + self.retention * self.velocity;

        if self.attraction.norm() != 0.0 {
            let av = self.attraction_point - *pos;

            if self.attraction_exponent == 1.0 {
                self.velocity += self.attraction * av * delta;
            } else {
                let m = av.norm().powf(self.attraction_exponent - 0.5);
                self.velocity += self.attraction * av * m * delta;
            }
        }

        orig_velocity
    }

    pub fn move_next(
        pos: Complex<f32>,
        mut move_params: MoveParams,
        delta_time: f32,
    ) -> MoveParams {
        move_params.update(&mut Complex::new(pos.re, pos.im), delta_time);
        move_params
    }

    pub fn move_asymptotic(
        vel0: Complex<f32>,
        vel1: Complex<f32>,
        retention: Complex<f32>,
    ) -> MoveParams {
        MoveParams {
            velocity: vel0,
            acceleration: vel1 * (Complex::new(1.0, 0.0) - retention),
            retention: retention.re,
            attraction: Complex::new(0.0, 0.0),
            attraction_point: Complex::new(0.0, 0.0),
            attraction_exponent: 1.0,
        }
    }

    pub fn move_asymptotic_halflife(
        vel0: Complex<f32>,
        vel1: Complex<f32>,
        halflife: f32,
    ) -> MoveParams {
        let retention = Complex::new(2.0_f32.powf(-1.0 / halflife), 0.0);
        Self::move_asymptotic(vel0, vel1, retention)
    }

    pub fn move_asymptotic_simple(vel: Complex<f32>, boost_factor: f32) -> MoveParams {
        let retention = 0.8;
        Self::move_asymptotic(
            vel * (Complex::new(1.0 + boost_factor, 0.0)),
            vel,
            Complex::new(retention, 0.0),
        )
    }

    pub fn move_from_towards(
        origin: Complex<f32>,
        target: Complex<f32>,
        attraction: Complex<f32>,
    ) -> MoveParams {
        let towards_move = Self::move_towards(Complex::new(0.0, 0.0), target, attraction);
        Self::move_next(origin, towards_move, 0.0)
    }

    pub fn move_towards_exp(
        vel: Complex<f32>,
        target: Complex<f32>,
        attraction: Complex<f32>,
        exponent: f32,
    ) -> MoveParams {
        MoveParams {
            velocity: vel,
            acceleration: Complex::new(0.0, 0.0),
            retention: 1.0,
            attraction,
            attraction_point: target,
            attraction_exponent: exponent,
        }
    }

    pub fn move_from_towards_exp(
        origin: Complex<f32>,
        target: Complex<f32>,
        attraction: Complex<f32>,
        exponent: f32,
    ) -> MoveParams {
        let towards_exp_move =
            Self::move_towards_exp(Complex::new(0.0, 0.0), target, attraction, exponent);
        Self::move_next(origin, towards_exp_move, 0.0)
    }

    pub fn move_dampen(vel: Complex<f32>, retention: f32) -> MoveParams {
        MoveParams {
            velocity: vel,
            acceleration: Complex::new(0.0, 0.0),
            retention,
            attraction: Complex::new(0.0, 0.0),
            attraction_point: Complex::new(0.0, 0.0),
            attraction_exponent: 1.0,
        }
    }

    pub fn move_towards(
        vel: Complex<f32>,
        target: Complex<f32>,
        attraction: Complex<f32>,
    ) -> MoveParams {
        MoveParams {
            velocity: vel,
            acceleration: Complex::new(0.0, 0.0),
            retention: 1.0,
            attraction,
            attraction_point: target,
            attraction_exponent: 1.0,
        }
    }

    pub fn move_linear(vel: Complex<f32>) -> MoveParams {
        MoveParams {
            velocity: vel,
            acceleration: Complex::new(0.0, 0.0),
            retention: 1.0,
            attraction: Complex::new(0.0, 0.0),
            attraction_point: Complex::new(0.0, 0.0),
            attraction_exponent: 1.0,
        }
    }
    pub fn move_accelerated(vel: Complex<f32>, accel: Complex<f32>) -> MoveParams {
        MoveParams {
            velocity: vel,
            acceleration: accel,
            retention: 1.0,
            attraction: Complex::new(0.0, 0.0),
            attraction_point: Complex::new(0.0, 0.0),
            attraction_exponent: 1.0,
        }
    }
}
