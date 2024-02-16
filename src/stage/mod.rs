use macroquad::prelude::*;
use num_complex::Complex;

use crate::{
    assets::AssetsHandler,
    components::{Hitpoint, Movement, MovementQueue, Position},
    engine::{
        spawner::{SpawnEvent, Spawner},
        stage::{PreloadType, Stage},
    },
    entity::{spawn_enemy, spawn_player},
};

pub fn stage_demo() -> Stage<'static> {
    let spawner = Spawner::new(vec![
        SpawnEvent::new(0.0, |world, assets_manager| {
            spawn_player(
                world,
                assets_manager
                    .textures
                    .get("remilia0")
                    .expect("There is no Remilia Texture"),
            );
        }),
        SpawnEvent::new(2.0, |world, assets_manager| {
            let pos = vec![
                Movement::new(Complex::new(0.1, 0.5), 2.0, false),
                Movement::new(Complex::new(0.5, 0.0), 0.0, true),
            ];
            let movement = MovementQueue::new(pos);
            let _ = spawn_enemy(
                world,
                Position::from_array([1.0, 0.1]),
                assets_manager
                    .textures
                    .get("fairy0")
                    .expect("There is no Fairy Texture"),
                movement,
                Hitpoint::new(10.0),
            );
        }),
    ]);

    Stage::new(
        "Demo",
        "Demo",
        vec![
            PreloadType::Texture("remilia0", "./resources/textures/remilia-scarlet/1.png"),
            PreloadType::Texture("fairy0", "./resources/textures/fairy/fairy0001.png"),
            PreloadType::Texture("hud", "./resources/ui/hud.png"),
            PreloadType::Texture("stage1", "./resources/background/stage-1.png"),
            PreloadType::Texture("mask", "./resources/ui/playable-mask.png"),
            PreloadType::Texture(
                "bullet0",
                "./resources/textures/projectiles/generic-bullet.png",
            ),
            PreloadType::Texture(
                "remi-bullet-0",
                "./resources/textures/projectiles/remi-bullet.png",
            ),
        ],
        spawner,
        |screen, assets_manager| {
            let offset = vec2(0.001, 0.001) * screen.playable_window().size().clone()
                + screen.playable_window().get_start().clone();
            let texture = assets_manager.textures.get("stage1").unwrap();
            let mask = assets_manager.textures.get("mask").unwrap();
            draw_texture_ex(
                &mask,
                offset.x,
                offset.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(screen.playable_window().size().clone()),
                    ..Default::default()
                },
            );
            draw_texture_ex(
                &texture,
                offset.x,
                offset.y,
                // WHITE,
                Color::new(1f32, 1f32, 1f32, 0.5),
                DrawTextureParams {
                    dest_size: Some(screen.playable_window().size().clone()),
                    ..Default::default()
                },
            );
        },
    )
}
