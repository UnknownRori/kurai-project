use macroquad::math::vec2;

use crate::math::ToVec2;

use super::transform2d::Transform2D;

#[derive(Debug, Clone, Copy)]
pub struct Hitbox {
    pub radius: f32,
}

impl Hitbox {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }

    pub fn is_intersect(
        &self,
        current_pos: &Transform2D,
        target_pos: &Transform2D,
        target_hitbox: &Self,
    ) -> bool {
        let distance_squared = vec2(current_pos.position().re, current_pos.position().im)
            .distance_squared(vec2(target_pos.position().re, target_pos.position().im));
        let sum_of_radii_squared = (self.radius + target_hitbox.radius).powi(2);
        distance_squared <= sum_of_radii_squared
    }
    pub fn near(
        &self,
        current_pos: &Transform2D,
        target_pos: &Transform2D,
        target_hitbox: &Self,
    ) -> bool {
        let distance_squared = current_pos
            .position()
            .to_vec2()
            .distance_squared(target_pos.position().to_vec2());
        let sum_of_radii_squared = (self.radius + 0.05 + target_hitbox.radius).powi(2);
        distance_squared <= sum_of_radii_squared
    }
}
