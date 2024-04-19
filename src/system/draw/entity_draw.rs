use hecs::World;
use macroquad::time::get_frame_time;
use num_traits::ToPrimitive;

use crate::{
    components::player::{Focus, Player},
    components::{
        death::DeathBlinkingAnimation, hitpoint::Damaged, sprite2d::Sprite2D,
        transform2d::Transform2D,
    },
    controls::Action,
    controls::Controls,
};

pub fn game_entity_draw(world: &World) {
    world
        .query::<(
            &Sprite2D,
            &Transform2D,
            Option<&Damaged>,
            Option<&DeathBlinkingAnimation>,
        )>()
        .iter()
        .for_each(|(_, (sprite2d, transform, damaged, death_blink))| {
            let mut opacity = None;

            if damaged.is_some() {
                println!("Get damaged!");
            }

            if death_blink.is_some() {
                if death_blink.unwrap().should_blink() {
                    opacity = Some(0.5);
                }
            }
            sprite2d.draw(&transform, opacity);
        });
}

pub fn player_focus_draw(world: &World, controls: &Controls<Action>, time: f64) {
    if controls.is_key_down(Action::Focus) {
        world
            .query::<(&Player, &Focus, &Transform2D)>()
            .iter()
            .for_each(|(_, (_, focus, transform))| {
                focus.0.draw(transform, None);

                let transform = Transform2D {
                    rotation: time.to_f32().unwrap_or(0.) % 360.0 * 2.,
                    ..*transform
                };
                focus.0.draw(&transform, None);
            });
    }
}
