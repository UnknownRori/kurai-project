use hecs::World;
use macroquad::audio::{play_sound, PlaySoundParams};

use crate::{
    assets::{AssetsHandler as _, AssetsManager},
    components::Position,
    entity::spawn_generic_bullet,
};

pub fn target_player_attack(
    world: &mut World,
    assets_manager: &AssetsManager,
    current: &Position,
    player: &Position,
    bullet_speed: f32,
) {
    let texture = assets_manager
        .textures
        .get("bullet-red")
        .expect("No generic bullet texture!");
    spawn_generic_bullet(world, &current, player, bullet_speed, texture);
    let sound = assets_manager.sfx.get("generic-shoot").unwrap();
    play_sound(
        &*sound,
        PlaySoundParams {
            looped: false,
            volume: 0.5,
        },
    );
}
