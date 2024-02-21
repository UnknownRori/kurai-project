use std::sync::{Arc, RwLock};

use keyframe::{
    functions::{Hold, Linear, Step},
    keyframes, AnimationSequence, Keyframe,
};

use hecs::World;
use macroquad::prelude::*;
use num_complex::Complex;
use num_traits::ToPrimitive;

use crate::{
    assets::{AssetsHandler, AssetsManager},
    components::{Hitpoint, Movement, MovementQueue, Position},
    engine::{
        spawner::{SpawnEvent, Spawner},
        stage::{PreloadType, Stage},
    },
    entity::{spawn_enemy, spawn_player},
};

fn spawner_line(
    start: f64,
    delay_between: f64,
    total: usize,
    spawn_event: impl FnMut(&mut World, &AssetsManager) + 'static,
) -> Vec<SpawnEvent> {
    let spawn_event = Arc::new(RwLock::new(spawn_event));
    let mut temp = Vec::with_capacity(total);

    for i in 0..total {
        let spawn_clone = Arc::clone(&spawn_event);
        temp.push(SpawnEvent::new2(
            start + (delay_between * i.to_f64().unwrap()),
            spawn_clone,
        ));
    }
    temp
}

pub fn stage_demo() -> Stage<'static> {
    // TODO : Refactor later
    let mut first_col = spawner_line(2., 1., 4, |world, assets_manager| {
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
            Hitpoint::new(2.5),
            0.4,
        );
    });
    let mut second_col = spawner_line(7.0, 0.8, 3, |world, assets_manager| {
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
            Hitpoint::new(2.5),
            0.5,
        );
    });

    let mut third_col = spawner_line(10., 1., 3, |world, assets_manager| {
        let pos = vec![
            Movement::new(Complex::new(1.0, 0.35), 0.0, true),
            Movement::new(Complex::new(0.0, 0.2), 0.0, false),
        ];
        let movement = MovementQueue::new(pos);
        let _ = spawn_enemy(
            world,
            Position::from_array([1.05, 0.2]),
            assets_manager
                .textures
                .get("fairy0")
                .expect("There is no Fairy Texture"),
            movement,
            Hitpoint::new(2.5),
            0.5,
        );
    });

    let mut timeline = vec![SpawnEvent::new(0.0, |world, assets_manager| {
        spawn_player(
            world,
            assets_manager
                .textures
                .get("remilia0")
                .expect("There is no Remilia Texture"),
        );
    })];

    timeline.append(&mut first_col);
    timeline.append(&mut second_col);
    timeline.append(&mut third_col);

    let spawner = Spawner::new(timeline);

    Stage::new(
        "Demo",
        "Demo",
        vec![
            PreloadType::Texture("remilia0", "./resources/textures/remilia-scarlet/1.png"),
            PreloadType::Texture("fairy0", "./resources/textures/fairy/fairy0001.png"),
            PreloadType::Texture("hud", "./resources/ui/hud.png"),
            PreloadType::Texture("stage1-bg-fog", "./resources/background/stage1-bg-fog.png"),
            PreloadType::Texture(
                "stage1-bg-water1",
                "./resources/background/stage1-bg-water1.png",
            ),
            PreloadType::Texture(
                "stage1-bg-water2",
                "./resources/background/stage1-bg-water2.png",
            ),
            PreloadType::Texture("mask", "./resources/ui/playable-mask.png"),
            PreloadType::Texture(
                "bullet-red",
                "./resources/textures/projectiles/generic-bullet-red.png",
            ),
            PreloadType::Texture(
                "bullet-blue",
                "./resources/textures/projectiles/generic-bullet-blue.png",
            ),
            PreloadType::Texture(
                "bullet-green",
                "./resources/textures/projectiles/generic-bullet-green.png",
            ),
            PreloadType::Texture(
                "remi-bullet-0",
                "./resources/textures/projectiles/remi-bullet.png",
            ),
            PreloadType::Texture(
                "remi-bullet-0",
                "./resources/textures/projectiles/remi-bullet.png",
            ),
            PreloadType::Texture("value", "./resources/textures/items/point.png"),
            PreloadType::Texture("power", "./resources/textures/items/power.png"),
            PreloadType::Sfx("generic-shoot", "./resources/sfx/generic-shoot.ogg"),
            PreloadType::Sfx("player-shoot", "./resources/sfx/player-shoot.ogg"),
            PreloadType::Sfx("player-death", "./resources/sfx/death.ogg"),
            PreloadType::Bgm(
                "title-screen",
                "./resources/music/Mysterious Crimson Dream.ogg",
            ),
        ],
        String::from("title-screen"),
        spawner,
        |time, screen, assets_manager| {
            let offset = vec2(0.001, 0.001) * screen.playable_window().size().clone()
                + screen.playable_window().get_start().clone();

            let mut keyframe_bg1 = keyframes![(0.2, 0.0), (0.5, 1.0), (0.2, 2.0)];
            let mut keyframe_bg2 = keyframes![(0.5, 0.0), (0.2, 1.0), (0.5, 2.0)];
            keyframe_bg1.advance_by(time % 2.0);
            keyframe_bg2.advance_by(time % 2.0);

            let stage1_bg1 = assets_manager.textures.get("stage1-bg-water1").unwrap();
            let stage1_bg2 = assets_manager.textures.get("stage1-bg-water2").unwrap();
            let stage1_fog = assets_manager.textures.get("stage1-bg-fog").unwrap();
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
            // INFO : First bg
            let bg1_y = offset.y - screen.playable_window().size().y
                + ((time.to_f32().unwrap() % 19.5) * screen.playable_window().size().y / 9.75);
            draw_texture_ex(
                &stage1_bg1,
                offset.x,
                bg1_y,
                Color::new(1f32, 1f32, 1f32, keyframe_bg1.now()),
                DrawTextureParams {
                    dest_size: Some(screen.playable_window().size().clone()),
                    ..Default::default()
                },
            );

            draw_texture_ex(
                &stage1_bg2,
                offset.x,
                bg1_y,
                Color::new(1f32, 1f32, 1f32, keyframe_bg2.now()),
                DrawTextureParams {
                    dest_size: Some(screen.playable_window().size().clone()),
                    ..Default::default()
                },
            );

            // INFO : bg2
            let bg2_y = offset.y - screen.playable_window().size().y
                + (((time.to_f32().unwrap() + 9.75) % 19.5) * screen.playable_window().size().y
                    / 9.75);
            draw_texture_ex(
                &stage1_bg1,
                offset.x,
                bg2_y,
                Color::new(1f32, 1f32, 1f32, keyframe_bg1.now()),
                DrawTextureParams {
                    dest_size: Some(screen.playable_window().size().clone()),
                    ..Default::default()
                },
            );

            draw_texture_ex(
                &stage1_bg2,
                offset.x,
                bg2_y,
                Color::new(1f32, 1f32, 1f32, keyframe_bg2.now()),
                DrawTextureParams {
                    dest_size: Some(screen.playable_window().size().clone()),
                    ..Default::default()
                },
            );

            let mut fog_half = screen.playable_window().size().clone();
            fog_half.y /= 1.35;
            draw_texture_ex(
                &stage1_fog,
                offset.x,
                offset.y,
                Color::new(1f32, 1f32, 1f32, 0.3),
                DrawTextureParams {
                    dest_size: Some(fog_half),
                    ..Default::default()
                },
            );
        },
    )
}
