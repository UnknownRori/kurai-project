use macroquad::{
    material::MaterialParams,
    miniquad::{self, BlendFactor, BlendState, Equation, PipelineParams, UniformType},
};

use crate::assets::AssetsManager;

use super::{
    konst::{
        BLOOM_MATERIAL, FAIRY_1, FOCUS, GENERIC_SHOOT_SOUND, GRAZE, LIGHTMAP, PICHUN,
        POST_PROCESSING, RED_BULLET, REMILIA_TEXTURE_1, REMI_BULLET_1, STAGE_1_BG_SHADER,
        STAGE_1_GROUND, SUPER_PERLIN, TEXTURE_HUD,
    },
    preload_sfx, preload_texture,
};

// TODO : Make this lazy load at specific stage only if the texture only used on that stage
pub async fn preload(assets_manager: &mut AssetsManager) {
    preload_texture(assets_manager, TEXTURE_HUD, "./resources/ui/hud.png").await;
    preload_texture(
        assets_manager,
        FOCUS,
        "./resources/textures/parts/focus.png",
    )
    .await;
    preload_texture(
        assets_manager,
        REMILIA_TEXTURE_1,
        "./resources/textures/remilia-scarlet/1.png",
    )
    .await;
    preload_texture(
        assets_manager,
        FAIRY_1,
        "./resources/textures/fairy/fairy0001.png",
    )
    .await;

    preload_texture(
        assets_manager,
        RED_BULLET,
        "./resources/textures/projectiles/generic-bullet-red.png",
    )
    .await;

    preload_texture(
        assets_manager,
        REMI_BULLET_1,
        "./resources/textures/projectiles/remi-bullet.png",
    )
    .await;

    preload_sfx(
        assets_manager,
        GENERIC_SHOOT_SOUND,
        "./resources/sfx/generic-shoot.ogg",
    )
    .await;
    preload_sfx(assets_manager, PICHUN, "./resources/sfx/death.ogg").await;
    preload_sfx(assets_manager, GRAZE, "./resources/sfx/graze.ogg").await;

    preload_texture(
        assets_manager,
        STAGE_1_GROUND,
        "./resources/background/stage1/ground.png",
    )
    .await;

    preload_texture(
        assets_manager,
        SUPER_PERLIN,
        "./resources/noise/super-perlin.png",
    )
    .await;

    assets_manager
        .shaders
        .register(
            STAGE_1_BG_SHADER,
            "./resources/shaders/stage1.vert.glsl",
            "./resources/shaders/stage1.frag.glsl",
            MaterialParams {
                uniforms: vec![
                    (String::from("iTime"), UniformType::Float1),
                    (String::from("iResolution"), UniformType::Float2),
                ],
                textures: vec![
                    String::from("noise_texture"),
                    String::from("entities_buffer"),
                ],
                ..Default::default()
            },
        )
        .await
        .unwrap();

    assets_manager
        .shaders
        .register(
            LIGHTMAP,
            "./resources/shaders/lightmap.vert.glsl",
            "./resources/shaders/lightmap.frag.glsl",
            MaterialParams {
                uniforms: vec![(String::from("iResolution"), UniformType::Float2)],
                pipeline_params: PipelineParams {
                    color_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::Value(miniquad::BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                    )),
                    alpha_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::One,
                        BlendFactor::One,
                    )),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .await
        .unwrap();

    assets_manager
        .shaders
        .register(
            BLOOM_MATERIAL,
            "./resources/shaders/bloom.vert.glsl",
            "./resources/shaders/bloom.frag.glsl",
            MaterialParams {
                uniforms: vec![
                    (String::from("iResolution"), UniformType::Float2),
                    (String::from("horizontal"), UniformType::Int1),
                ],
                pipeline_params: PipelineParams {
                    color_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::Value(miniquad::BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                    )),
                    alpha_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::One,
                        BlendFactor::One,
                    )),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .await
        .unwrap();

    assets_manager
        .shaders
        .register(
            POST_PROCESSING,
            "./resources/shaders/post_processing.vert.glsl",
            "./resources/shaders/post_processing.frag.glsl",
            MaterialParams {
                uniforms: vec![
                    (String::from("iResolution"), UniformType::Float2),
                    (String::from("exposure"), UniformType::Float1),
                ],
                textures: vec![String::from("bloom"), String::from("stage")],
                pipeline_params: PipelineParams {
                    color_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::Value(miniquad::BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                    )),
                    alpha_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::One,
                        BlendFactor::One,
                    )),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .await
        .unwrap();
}
