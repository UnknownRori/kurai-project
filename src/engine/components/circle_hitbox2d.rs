use macroquad::math::vec2;

use super::Transform2D;

pub struct CircleHitbox2D {
    radius: f32,
}

impl CircleHitbox2D {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }

    pub fn is_intersect_with_circle_hitbox(
        &self,
        current_pos: &Transform2D,
        target_pos: &Transform2D,
        target_hitbox: &CircleHitbox2D,
    ) -> bool {
        let distance_squared = vec2(current_pos.position().re, current_pos.position().im)
            .distance_squared(vec2(target_pos.position().re, target_pos.position().im));
        let sum_of_radii_squared = (self.radius + target_hitbox.radius).powi(2);
        distance_squared <= sum_of_radii_squared
    }
}
