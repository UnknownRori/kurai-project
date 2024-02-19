use hecs::World;
use macroquad::prelude::*;
use num_complex::Complex;

use crate::{
    assets::{AssetsHandler, AssetsManager},
    components::{Hitpoint, Movement, MovementQueue, Position},
    engine::{
        spawner::{SpawnEvent, Spawner},
        stage::{PreloadType, Stage},
    },
    entity::{spawn_enemy, spawn_player},
};

pub fn stage_demo() -> Stage<'static> {
    // TODO : Refactor later
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
                Movement::new(Complex::new(0.3, 0.5), 0.0, false),
                Movement::new(Complex::new(0.0, 0.8), 0.0, true),
            ];
            let movement = MovementQueue::new(pos);
            let _ = spawn_enemy(
                world,
                Position::from_array([0.3, -0.05]),
                assets_manager
                    .textures
                    .get("fairy0")
                    .expect("There is no Fairy Texture"),
                movement,
                Hitpoint::new(5.0),
                0.4,
            );
        }),
        SpawnEvent::new(3.0, |world, assets_manager| {
            let pos = vec![
                Movement::new(Complex::new(0.3, 0.5), 0.0, false),
                Movement::new(Complex::new(0.0, 0.8), 0.0, true),
            ];
            let movement = MovementQueue::new(pos);
            let _ = spawn_enemy(
                world,
                Position::from_array([0.3, -0.05]),
                assets_manager
                    .textures
                    .get("fairy0")
                    .expect("There is no Fairy Texture"),
                movement,
                Hitpoint::new(5.0),
                0.4,
            );
        }),
        SpawnEvent::new(2.5, |world, assets_manager| {
            let pos = vec![
                Movement::new(Complex::new(0.7, 0.5), 0.0, false),
                Movement::new(Complex::new(1.0, 0.8), 0.0, true),
            ];
            let movement = MovementQueue::new(pos);
            let _ = spawn_enemy(
                world,
                Position::from_array([0.7, -0.05]),
                assets_manager
                    .textures
                    .get("fairy0")
                    .expect("There is no Fairy Texture"),
                movement,
                Hitpoint::new(5.0),
                0.4,
            );
        }),
        SpawnEvent::new(3.5, |world, assets_manager| {
            let pos = vec![
                Movement::new(Complex::new(0.7, 0.5), 0.0, false),
                Movement::new(Complex::new(1.0, 0.8), 0.0, true),
            ];
            let movement = MovementQueue::new(pos);
            let _ = spawn_enemy(
                world,
                Position::from_array([0.7, -0.05]),
                assets_manager
                    .textures
                    .get("fairy0")
                    .expect("There is no Fairy Texture"),
                movement,
                Hitpoint::new(5.0),
                0.4,
            );
        }),
        SpawnEvent::new(7.0, |world, assets_manager| {
            let pos = vec![
                Movement::new(Complex::new(0.0, 0.2), 0.0, false),
                Movement::new(Complex::new(1.0, 0.35), 0.0, true),
            ];
            let movement = MovementQueue::new(pos);
            let _ = spawn_enemy(
                world,
                Position::from_array([-0.05, 0.2]),
                assets_manager
                    .textures
                    .get("fairy0")
                    .expect("There is no Fairy Texture"),
                movement,
                Hitpoint::new(5.0),
                0.5,
            );
        }),
        SpawnEvent::new(7.8, |world, assets_manager| {
            let pos = vec![
                Movement::new(Complex::new(0.0, 0.2), 0.0, false),
                Movement::new(Complex::new(1.0, 0.35), 0.0, true),
            ];
            let movement = MovementQueue::new(pos);
            let _ = spawn_enemy(
                world,
                Position::from_array([-0.05, 0.2]),
                assets_manager
                    .textures
                    .get("fairy0")
                    .expect("There is no Fairy Texture"),
                movement,
                Hitpoint::new(5.0),
                0.5,
            );
        }),
        SpawnEvent::new(8.6, |world, assets_manager| {
            let pos = vec![
                Movement::new(Complex::new(0.0, 0.2), 0.0, false),
                Movement::new(Complex::new(1.0, 0.35), 0.0, true),
            ];
            let movement = MovementQueue::new(pos);
            let _ = spawn_enemy(
                world,
                Position::from_array([-0.05, 0.2]),
                assets_manager
                    .textures
                    .get("fairy0")
                    .expect("There is no Fairy Texture"),
                movement,
                Hitpoint::new(5.0),
                0.5,
            );
        }),
        SpawnEvent::new(9.4, |world, assets_manager| {
            let pos = vec![
                Movement::new(Complex::new(0.0, 0.2), 0.0, false),
                Movement::new(Complex::new(1.0, 0.35), 0.0, true),
            ];
            let movement = MovementQueue::new(pos);
            let _ = spawn_enemy(
                world,
                Position::from_array([-0.05, 0.2]),
                assets_manager
                    .textures
                    .get("fairy0")
                    .expect("There is no Fairy Texture"),
                movement,
                Hitpoint::new(5.0),
                0.5,
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
            PreloadType::Sfx("generic-shoot", "./resources/sfx/generic-shoot.ogg"),
            PreloadType::Sfx("player-shoot", "./resources/sfx/player-shoot.ogg"),
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
