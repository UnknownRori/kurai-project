use hecs::World;

use crate::engine::components::{Sprite2D, Transform2D};

pub fn sprite2d_draw(world: &World) {
    world
        .query::<(&Sprite2D, &Transform2D)>()
        .iter()
        .for_each(|(_, (sprite2d, transform))| {
            sprite2d.draw(&transform);
        });
}
