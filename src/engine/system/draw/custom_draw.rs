use hecs::World;

use crate::engine::components::CustomDraw;

pub fn custom_draw(world: &World) {
    world
        .query::<(&CustomDraw)>()
        .iter()
        .for_each(|(_, (draw))| (draw.0)());
}
