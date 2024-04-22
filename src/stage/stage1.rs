use std::sync::{Arc, Mutex};

use hecs::World;
use macroquad::prelude::*;

use crate::{
    assets::{
        konst::{FAIRY, PERLIN, STAGE1_BG_SHADERS, STAGE1_GROUND},
        Assets,
    },
    attack_info::non_spells::FairyBurst,
    cmpx,
    components::{
        AttackInfo, DropItem, Enemy, Hitbox, Hitpoint, MoveParams, Sprite, Transform2D, Waypoint,
        WaypointFactor, Waypoints,
    },
    entity::{player_spawn, SpawnEvent, Spawner},
    konst::{VIRTUAL_STAGE_HEIGHT, VIRTUAL_STAGE_WIDTH},
    vec2, Game,
};

use super::{ActiveStage, LazyStage, StageType};

pub struct LazyStage1;

impl LazyStage for LazyStage1 {
    fn build(&self, game: &Game) -> Box<dyn super::ActiveStage> {
        let spawner = Spawner::new(vec![
            SpawnEvent::new(0., player_spawn),
            SpawnEvent::new(3., spawn_fairy_burst_1),
            SpawnEvent::new(4., spawn_fairy_burst_1),
            SpawnEvent::new(5., spawn_fairy_burst_1),
            SpawnEvent::new(8., spawn_fairy_burst_2),
            SpawnEvent::new(9., spawn_fairy_burst_2),
            SpawnEvent::new(10., spawn_fairy_burst_2),
        ]);

        let bg_material = game
            .assets
            .materials
            .get(STAGE1_BG_SHADERS)
            .unwrap()
            .clone();
        let bg_texture = game.assets.textures.get(STAGE1_GROUND).unwrap().clone();
        let bg_noise = game.assets.textures.get(PERLIN).unwrap();

        bg_material.set_uniform(
            "iResolution",
            vec2(VIRTUAL_STAGE_WIDTH as f32, VIRTUAL_STAGE_HEIGHT as f32),
        );
        bg_material.set_texture("noise_texture", bg_noise.clone());

        Box::new(Stage1 {
            bg_material,
            bg_texture,
            spawner,
            timer: 0.,
        })
    }
}

pub struct Stage1 {
    pub bg_material: Material,
    pub bg_texture: Texture2D,

    spawner: Spawner,
    timer: f32,
}

impl ActiveStage for Stage1 {
    fn title(&self) -> &str {
        "Stage 1"
    }

    fn sub_title(&self) -> &str {
        "Misty Lake"
    }

    fn stage_type(&self) -> &StageType {
        &StageType::Story
    }

    fn update(&mut self, game: &mut crate::Game) {
        self.timer += get_frame_time();
        self.spawner
            .update(&mut game.world, &game.assets, get_frame_time());
    }

    fn draw(&self, game: &crate::Game) {
        self.bg_material.set_uniform("iTime", self.timer);
        self.bg_material.set_texture(
            "entities_buffer",
            game.renderer
                .entities
                .render_target
                .as_ref()
                .unwrap()
                .texture
                .clone(),
        );
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
}

fn spawn_fairy_burst_1(world: &mut World, assets: &Assets) {
    let sprite = assets.textures.get(FAIRY).unwrap();
    let transform = Transform2D::new(cmpx!(0.35, -0.2), vec2!(0.11), 0.);
    let attack = AttackInfo::new(Arc::new(Mutex::new(FairyBurst::new(assets, 2.))));
    let waypoint = Waypoints::new(vec![
        Waypoint::new(
            5.,
            WaypointFactor::PreserveVelocity(MoveParams::move_from_towards(
                cmpx!(0.35, 0.5),
                cmpx!(0.9, 0.0),
                cmpx!(0.2, 0.1),
            )),
        ),
        Waypoint::new(
            2.,
            WaypointFactor::PreserveVelocity(MoveParams::move_from_towards(
                cmpx!(1., 1.),
                cmpx!(0.9, 0.0),
                cmpx!(0.02, 0.01),
            )),
        ),
    ]);
    let drop = DropItem {
        power: 1,
        value: 2,
        ..Default::default()
    };

    world.spawn((
        Enemy,
        attack,
        transform,
        drop,
        waypoint,
        Hitbox::new(0.015),
        Hitpoint::new(2.5),
        Sprite::new(sprite.clone()),
        MoveParams::move_linear(cmpx!(0., 0.1)),
    ));
}

fn spawn_fairy_burst_2(world: &mut World, assets: &Assets) {
    let sprite = assets.textures.get(FAIRY).unwrap();
    let transform = Transform2D::new(cmpx!(0.65, -0.2), vec2!(0.11), 0.);
    let attack = AttackInfo::new(Arc::new(Mutex::new(FairyBurst::new(assets, 2.))));
    let waypoint = Waypoints::new(vec![
        Waypoint::new(
            5.,
            WaypointFactor::PreserveVelocity(MoveParams::move_from_towards(
                cmpx!(-0.35, 0.5),
                cmpx!(-0.5, 0.0),
                cmpx!(0.2, 0.01),
            )),
        ),
        Waypoint::new(
            2.,
            WaypointFactor::PreserveVelocity(MoveParams::move_from_towards(
                cmpx!(-1., 1.),
                cmpx!(-0.5, 0.0),
                cmpx!(0.02, 0.005),
            )),
        ),
    ]);
    let drop = DropItem {
        power: 1,
        value: 2,
        ..Default::default()
    };

    world.spawn((
        Enemy,
        attack,
        transform,
        drop,
        waypoint,
        Hitbox::new(0.015),
        Hitpoint::new(2.5),
        Sprite::new(sprite.clone()),
        MoveParams::move_linear(cmpx!(0., 0.1)),
    ));
}
