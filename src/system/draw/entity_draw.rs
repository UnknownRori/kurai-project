use hecs::World;
use num_traits::ToPrimitive;

use crate::{
    components::{
        hud::HUD,
        player::{Focus, Player},
    },
    controls::Action,
    engine::{
        components::{Sprite2D, Transform2D},
        controls::Controls,
    },
};

pub fn game_entity_draw(world: &World) {
    world
        .query::<(&Sprite2D, &Transform2D)>()
        .without::<&HUD>()
        .iter()
        .for_each(|(_, (sprite2d, transform))| {
            sprite2d.draw(&transform);
        });
}

pub fn player_focus_draw(world: &World, controls: &Controls<Action>, time: f64) {
    if controls.is_key_down(Action::Focus) {
        world
            .query::<(&Player, &Focus, &Transform2D)>()
            .without::<&HUD>()
            .iter()
            .for_each(|(_, (_, focus, transform))| {
                focus.0.draw(transform);

                let transform = Transform2D {
                    rotation: time.to_f32().unwrap_or(0.) % 360.0 * 2.,
                    ..*transform
                };
                focus.0.draw(&transform);
            });
    }
}
