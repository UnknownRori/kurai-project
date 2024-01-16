use hecs::World;

use crate::{
    controls::{Action, Controls},
    entity::PlayerEntity,
};

pub mod player;

pub fn update_player_move(world: &mut World, controls: &Controls) {
    for (_, (_, _, _)) in &mut world.query::<PlayerEntity>() {
        if controls.is_down(&Action::Left) {
            tracing::info!("Player pressed : Left!");
        }
    }
}
