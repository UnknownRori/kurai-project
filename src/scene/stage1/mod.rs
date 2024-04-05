use std::sync::Arc;

use hecs::World;
use macroquad::prelude::*;
use num_traits::ToPrimitive as _;

use crate::{
    assets::konst::{FAIRY_1, STAGE_1_BG_SHADER, STAGE_1_GROUND, SUPER_PERLIN},
    cmpx,
    components::{
        movement::MoveParams,
        waypoint::{Waypoint, WaypointFactor},
        waypoints::Waypoints,
    },
    engine::{
        components::{Hitpoint, Sprite2D, Transform2D},
        ecs::{SpawnEvent, Spawner},
    },
    entity::{lazy_spawn_enemy, lazy_spawn_enemy2, player::lazy_spawn_player},
    konst::{VIRTUAL_STAGE_HEIGHT, VIRTUAL_STAGE_WIDTH},
};

use super::{lazy_stage::LazyStage, scene::Scene, stage::Stage, stage_info::StageInfo};

pub struct Stage1 {
    pub bg_material: Material,
    pub bg_texture: Texture2D,

    pub spawner: Spawner,
}

impl Stage for Stage1 {}

impl StageInfo for Stage1 {
    fn id(&self) -> usize {
        1
    }

    fn title(&self) -> &str {
        "Stage 1"
    }

    fn sub_title(&self) -> &str {
        "Misty Lake"
    }

    fn stage_type(&self) -> &super::stage_info::StageType {
        &super::stage_info::StageType::Story
    }
}

impl Scene for Stage1 {
    fn start(&mut self, time: f64, delta: f32) {
        //
    }

    fn update(&mut self, world: &mut World, time: f64, delta: f32) {
        self.spawner.update(world, delta.into());
    }

    fn draw(&self, time: f64, delta: f32) {
        self.bg_material
            .set_uniform("iTime", time.to_f32().unwrap());
        gl_use_material(&self.bg_material);
        draw_texture_ex(
            &self.bg_texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(1., 1.)),
                ..Default::default()
            },
        );
        gl_use_default_material();
    }

    fn end(&mut self) {
        //
    }
}

pub struct Stage1Lazy;

impl StageInfo for Stage1Lazy {
    fn id(&self) -> usize {
        1
    }

    fn title(&self) -> &str {
        "Stage 1"
    }

    fn sub_title(&self) -> &str {
        "Misty Lake"
    }

    fn stage_type(&self) -> &super::stage_info::StageType {
        &super::stage_info::StageType::Story
    }
}

impl LazyStage for Stage1Lazy {
    fn build(&self, assets_manager: &crate::engine::assets::AssetsManager) -> Box<dyn Stage> {
        let player_spawn = lazy_spawn_player(assets_manager);

        let fairy = assets_manager.textures.get(FAIRY_1).unwrap();
        let fairy_spawn = lazy_spawn_enemy(
            assets_manager,
            Transform2D::new(cmpx!(0.2, 0.2), vec2(0.1, 0.1), 0.),
            Sprite2D::new(fairy.clone()),
            Hitpoint::new(2.5),
        );

        let fairy_spawn1 = lazy_spawn_enemy(
            assets_manager,
            Transform2D::new(cmpx!(0.8, 0.2), vec2(0.1, 0.1), 0.),
            Sprite2D::new(fairy.clone()),
            Hitpoint::new(2.5),
        );

        let fairy_spawn2 = lazy_spawn_enemy2(
            assets_manager,
            Transform2D::new(cmpx!(0.5, 0.3), vec2(0.1, 0.1), 0.),
            Sprite2D::new(fairy.clone()),
            Hitpoint::new(2.5),
            Waypoints::new(vec![Waypoint::new(
                1.,
                WaypointFactor::PreserveVelocity(MoveParams::move_accelerated(
                    cmpx!(0.),
                    cmpx!(0., 0.1),
                )),
            )]),
        );

        let spawn_lists = vec![
            SpawnEvent::new(
                0.,
                Arc::new(move |world| {
                    (player_spawn)(world);
                }),
            ),
            SpawnEvent::new(
                2.9,
                Arc::new(move |world| {
                    (fairy_spawn)(world);
                }),
            ),
            SpawnEvent::new(
                2.9,
                Arc::new(move |world| {
                    (fairy_spawn1)(world);
                }),
            ),
            SpawnEvent::new(
                4.,
                Arc::new(move |world| {
                    (fairy_spawn2)(world);
                }),
            ),
        ];

        let spawner = Spawner::new(spawn_lists);

        let bg_material = assets_manager.shaders.get(STAGE_1_BG_SHADER).unwrap();
        let bg_texture = assets_manager.textures.get(STAGE_1_GROUND).unwrap();
        let bg_noise = assets_manager.textures.get(SUPER_PERLIN).unwrap();

        bg_material.set_uniform(
            "iResolution",
            vec2(VIRTUAL_STAGE_WIDTH as f32, VIRTUAL_STAGE_HEIGHT as f32),
        );
        bg_material.set_texture("noise_texture", bg_noise.clone());

        Box::new(Stage1 {
            spawner,
            bg_texture,
            // bg_noise,
            bg_material,
        })
    }
}
