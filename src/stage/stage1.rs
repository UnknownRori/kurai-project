use std::sync::Arc;

use super::utils::spawner_line;
use keyframe::{keyframes, AnimationSequence};

use macroquad::prelude::*;
use num_complex::Complex;
use num_traits::ToPrimitive;

use crate::{
    assets::{AssetsHandler, AssetsManager},
    components::{Hitpoint, Movement, MovementQueue, Position},
    engine::{
        animation::BackgroundAtlasAnimation,
        spawner::{SpawnEvent, Spawner},
        stage::{PreloadType, Stage, StageBackground},
    },
    entity::{spawn_enemy, spawn_player},
};

struct Stage1Background {
    pub atlas: Arc<Texture2D>,
    pub bg_animation: BackgroundAtlasAnimation,
}

impl Stage1Background {
    pub fn new(assets_manager: &AssetsManager) -> Box<dyn StageBackground> {
        let atlas = assets_manager.textures.get("stage1").unwrap();
        let bg_animation =
            BackgroundAtlasAnimation::new(&atlas, 8, vec2(900., 1100.), 50, 20., 1., true);

        Box::new(Self {
            atlas,
            bg_animation,
        })
    }
}

impl StageBackground for Stage1Background {
    fn draw(&self, _time: f64, _delta: f32, screen: &crate::window::Window) {
        let offset = vec2(0.001, 0.001) * screen.playable_window().size().clone()
            + screen.playable_window().get_start().clone();

        draw_texture_ex(
            &*self.atlas,
            offset.x,
            offset.y,
            WHITE,
            DrawTextureParams {
                source: Some(*self.bg_animation.source_rect()),
                dest_size: Some(*screen.playable_window().size()),
                ..Default::default()
            },
        )
    }

    fn update(&mut self, time: f64, delta: f32) {
        self.bg_animation.update(time, delta);
    }
}

pub fn stage_1() -> Stage<'static> {
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
        "Misty Lake",
        "Stage 1",
        vec![
            PreloadType::Texture("remilia0", "./resources/textures/remilia-scarlet/1.png"),
            PreloadType::Texture("fairy0", "./resources/textures/fairy/fairy0001.png"),
            PreloadType::Texture("hud", "./resources/ui/hud.png"),
            PreloadType::Texture("stage1", "./resources/background/stage1.png"),
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
            PreloadType::Texture("focus", "./resources/textures/parts/focus.png"),
            // PreloadType::Texture("value", "./resources/textures/items/point.png"),
            // PreloadType::Texture("power", "./resources/textures/items/power.png"),
            PreloadType::Sfx("generic-shoot", "./resources/sfx/generic-shoot.ogg"),
            PreloadType::Sfx("player-shoot", "./resources/sfx/player-shoot.ogg"),
            PreloadType::Sfx("player-death", "./resources/sfx/death.ogg"),
            PreloadType::Sfx("graze", "./resources/sfx/graze.ogg"),
            PreloadType::Bgm(
                "title-screen",
                "./resources/music/Mysterious Crimson Dream.ogg",
            ),
        ],
        String::from("title-screen"),
        spawner,
        Stage1Background::new,
    )
}
