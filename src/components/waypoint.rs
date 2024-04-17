use crate::time::Timer;

use super::movement::MoveParams;

#[derive(Debug, Copy, Clone)]
pub enum WaypointFactor {
    PreserveVelocity(MoveParams),
    Overwrite(MoveParams),
}

#[derive(Debug, Clone)]
pub struct Waypoint {
    pub factor: WaypointFactor,
    pub time: Timer,
}

impl Waypoint {
    pub fn new(time: f32, factor: WaypointFactor) -> Self {
        Self {
            factor,
            time: Timer::new(time, false),
        }
    }
}
