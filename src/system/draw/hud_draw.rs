use hecs::World;

use crate::{
    components::hud::HUD,
    engine::components::{CustomDraw, Sprite2D, Transform2D},
};

pub fn hud_draw(world: &World) {
    world
        .query::<(&HUD, &Sprite2D, &Transform2D)>()
        .iter()
        .for_each(|(_, (_, sprite2d, transform))| {
            sprite2d.draw(&transform);
        });

    world
        .query::<(&HUD, &CustomDraw)>()
        .iter()
        .for_each(|(_, (_, custom))| (custom.0)());
}
