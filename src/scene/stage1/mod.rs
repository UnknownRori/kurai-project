use std::sync::Arc;

use hecs::World;
use macroquad::math::vec2;

use crate::{
    assets::konst::FAIRY_1,
    engine::{
        components::{Hitpoint, Movement, Transform2D},
        ecs::{SpawnEvent, Spawner},
        math::complx,
    },
    entity::{lazy_spawn_enemy, player::lazy_spawn_player},
};

use super::{lazy_stage::LazyStage, scene::Scene, stage::Stage, stage_info::StageInfo};

pub struct Stage1 {
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
        self.spawner.start(time);
    }

    fn update(&mut self, world: &mut World, time: f64, delta: f32) {
        self.spawner.update(world, time);
    }

    fn draw(&self, time: f64, delta: f32) {
        //
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
        let movement_queue = vec![
            Movement::new(complx(0.3, 0.5), 0.0, false),
            Movement::new(complx(0.0, 0.8), 0.0, true),
        ];
        let fairy_spawn = lazy_spawn_enemy(
            Transform2D::new(complx(0.2, -0.25), vec2(0.1, 0.1), 0.),
            fairy,
            movement_queue,
            Hitpoint::new(2.5),
        );

        let spawn_lists = vec![
            SpawnEvent::new(
                0.,
                Arc::new(move |world| {
                    (player_spawn)(world);
                }),
            ),
            SpawnEvent::new(
                1.,
                Arc::new(move |world| {
                    (fairy_spawn)(world);
                }),
            ),
        ];

        let spawner = Spawner::new(spawn_lists);

        Box::new(Stage1 { spawner })
    }
}
