use std::collections::VecDeque;

use super::{movement::MoveParams, waypoint::*};

#[derive(Debug, Clone)]
pub struct Waypoints {
    waypoints: VecDeque<Waypoint>,
}

impl Waypoints {
    pub fn new(waypoints: Vec<Waypoint>) -> Self {
        Self {
            waypoints: waypoints.into(),
        }
    }

    pub fn update(&mut self, move_params: &mut MoveParams) {
        if let Some(current_waypoint) = self.waypoints.front_mut() {
            current_waypoint.time.update();
            if !current_waypoint.time.completed() {
                return;
            }

            match current_waypoint.factor {
                WaypointFactor::Overwrite(param) => {
                    *move_params = param;
                }

                WaypointFactor::PreserveVelocity(mut param) => {
                    param.velocity = move_params.velocity;
                    *move_params = param;
                }
            }

            // We done modifying the target move_params
            self.waypoints.pop_front();
        }
    }
}
