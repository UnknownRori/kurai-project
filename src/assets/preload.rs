use macroquad::{material::MaterialParams, miniquad::UniformType};

use crate::engine::assets::AssetsManager;

use super::{
    konst::{
        FAIRY_1, FOCUS, GENERIC_SHOOT_SOUND, GRAZE, PICHUN, RED_BULLET, REMILIA_TEXTURE_1,
        REMI_BULLET_1, STAGE_1_BG_SHADER, STAGE_1_GROUND, TEXTURE_HUD,
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
                ..Default::default()
            },
        )
        .await
        .unwrap();
}
