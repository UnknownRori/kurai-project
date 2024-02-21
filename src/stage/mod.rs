use std::{
    rc::Rc,
    sync::{Arc, RwLock},
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
        ],
        spawner,
        |_, screen, assets_manager| {
            let offset = vec2(0.001, 0.001) * screen.playable_window().size().clone()
                + screen.playable_window().get_start().clone();
            let stage1_bg1 = assets_manager.textures.get("stage1-bg-water1").unwrap();
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
            draw_texture_ex(
                &stage1_bg1,
                offset.x,
                offset.y,
                // WHITE,
                Color::new(1f32, 1f32, 1f32, 0.5),
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
                // WHITE,
                Color::new(1f32, 1f32, 1f32, 0.3),
                DrawTextureParams {
                    dest_size: Some(fog_half),
                    ..Default::default()
                },
            );
        },
    )
}
