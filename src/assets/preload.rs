use macroquad::{
    material::MaterialParams,
    miniquad::{
        BlendFactor, BlendState, BlendValue, Equation, FilterMode, PipelineParams, UniformType,
    },
};

use super::{
    konst::{
        BLOOM, BULLET_RED, BULLET_REMI, FAIRY, FOCUS, HUD, LIGHTMAP, PERLIN, POINT,
        POST_PROCESSING, POWER, REMI, STAGE1_BG_SHADERS, STAGE1_GROUND,
    },
    Assets,
};

pub async fn preload(assets: &mut Assets) {
    assets
        .load_texture(
            "./resources/textures/remilia-scarlet/1.png",
            REMI,
            Some(FilterMode::Linear),
        )
        .await
        .unwrap();
    assets
        .load_texture("./resources/ui/hud.png", HUD, Some(FilterMode::Linear))
        .await
        .unwrap();
    assets
        .load_texture(
            "./resources/textures/parts/focus.png",
            FOCUS,
            Some(FilterMode::Linear),
        )
        .await
        .unwrap();

    assets
        .load_texture(
            "./resources/textures/items/point.png",
            POINT,
            Some(FilterMode::Linear),
        )
        .await
        .unwrap();
    assets
        .load_texture(
            "./resources/textures/items/power.png",
            POWER,
            Some(FilterMode::Linear),
        )
        .await
        .unwrap();

    assets
        .load_texture(
            "./resources/textures/fairy/fairy0001.png",
            FAIRY,
            Some(FilterMode::Linear),
        )
        .await
        .unwrap();
    assets
        .load_texture(
            "./resources/textures/projectiles/generic-bullet-red.png",
            BULLET_RED,
            Some(FilterMode::Linear),
        )
        .await
        .unwrap();
    assets
        .load_texture(
            "./resources/textures/projectiles/remi-bullet.png",
            BULLET_REMI,
            Some(FilterMode::Linear),
        )
        .await
        .unwrap();

    assets
        .load_texture(
            "./resources/background/stage1/ground.png",
            STAGE1_GROUND,
            Some(FilterMode::Linear),
        )
        .await
        .unwrap();

    assets
        .load_texture(
            "./resources/noise/super-perlin.png",
            PERLIN,
            Some(FilterMode::Linear),
        )
        .await
        .unwrap();

    assets
        .load_material(
            STAGE1_BG_SHADERS,
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

    assets
        .load_material(
            LIGHTMAP,
            "./resources/shaders/lightmap.vert.glsl",
            "./resources/shaders/lightmap.frag.glsl",
            MaterialParams {
                uniforms: vec![(String::from("iResolution"), UniformType::Float2)],
                pipeline_params: PipelineParams {
                    color_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::Value(BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
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

    assets
        .load_material(
            BLOOM,
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
                        BlendFactor::Value(BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
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

    assets
        .load_material(
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
                        BlendFactor::Value(BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
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
