use hecs::World;
use macroquad::prelude::*;

use crate::{
    assets::konst::POST_PROCESSING,
    controls::Action,
    engine::{
        assets::AssetsManager,
        camera::screen_buffer2d::{ScreenBuffer2D, ScreenBuffer2DBuilder},
        controls::Controls,
        fps_counter::FPSCounter,
    },
    konst::{
        DESIRED_ASPECT_RATIO, VIRTUAL_SCREEN_WIDTH, VIRTUAL_STAGE_HEIGHT, VIRTUAL_STAGE_WIDTH,
    },
    scene::stage::StageManager,
    score::ScoreData,
    shader::{fetch_lightmap, generate_bloom},
    system::{
        draw::entity_draw::{game_entity_draw, player_focus_draw},
        update_draw_hud,
    },
    ui::game_hud::draw_entity_number,
};

pub struct RenderingBuffer {
    // Entity drawed
    pub stage: ScreenBuffer2D,
    // Everything combine and ui on top of that
    pub ui: ScreenBuffer2D,
    // lightmap from stage render
    pub lightmap: ScreenBuffer2D,
    // Bloom data
    pub bloom: ScreenBuffer2D,
    // combining the stage and bloom
    pub post_processing: ScreenBuffer2D,
}

impl Default for RenderingBuffer {
    fn default() -> Self {
        let ui: ScreenBuffer2D =
            ScreenBuffer2DBuilder::from_aspect_ratio(VIRTUAL_SCREEN_WIDTH, DESIRED_ASPECT_RATIO)
                .filter(FilterMode::Nearest)
                .into();

        let stage: ScreenBuffer2D =
            ScreenBuffer2DBuilder::from_size(VIRTUAL_STAGE_WIDTH, VIRTUAL_STAGE_HEIGHT)
                .filter(FilterMode::Nearest)
                .into();

        let lightmap: ScreenBuffer2D =
            ScreenBuffer2DBuilder::from_size(VIRTUAL_STAGE_WIDTH, VIRTUAL_STAGE_HEIGHT)
                .filter(FilterMode::Nearest)
                .into();

        let bloom: ScreenBuffer2D =
            ScreenBuffer2DBuilder::from_size(VIRTUAL_STAGE_WIDTH, VIRTUAL_STAGE_HEIGHT)
                .filter(FilterMode::Nearest)
                .into();

        let post_processing: ScreenBuffer2D =
            ScreenBuffer2DBuilder::from_size(VIRTUAL_STAGE_WIDTH, VIRTUAL_STAGE_HEIGHT)
                .filter(FilterMode::Nearest)
                .into();

        Self {
            ui,
            bloom,
            stage,
            lightmap,
            post_processing,
        }
    }
}

pub fn draw_main_ui(
    world: &World,
    render: &RenderingBuffer,
    controls: &Controls<Action>,
    font: &Font,
    score: &ScoreData,
    fps_counter: &FPSCounter,
    time: f64,
    delta: f32,
) {
    render.ui.set_camera();
    clear_background(BLACK);

    let offset = vec2(0.03, 0.009);
    draw_texture_ex(
        render.post_processing.texture(),
        offset.x,
        offset.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(0.64, 0.975)),
            ..Default::default()
        },
    );

    update_draw_hud(world, controls, score, fps_counter, font, time, delta);
    draw_entity_number(world.len(), font);
    render.ui.done_camera();
}

pub fn draw_stage(
    world: &World,
    assets_manager: &AssetsManager,
    stage_manager: &StageManager,
    render: &RenderingBuffer,
    controls: &Controls<Action>,
    time: f64,
    delta: f32,
) {
    render.stage.set_camera();
    clear_background(Color::new(0., 0., 0., 0.));

    stage_manager.draw(time, delta);
    game_entity_draw(world);
    render.stage.done_camera();

    fetch_lightmap(assets_manager, &render.stage, &render.lightmap);
    generate_bloom(assets_manager, &render.lightmap, &render.bloom);

    render.post_processing.set_camera();
    clear_background(BLACK);

    let post_processing = assets_manager.shaders.get(POST_PROCESSING).unwrap();
    post_processing.set_uniform("iResolution", render.bloom.texture().size());
    post_processing.set_uniform("exposure", 0.8f32);
    post_processing.set_texture("bloom", render.bloom.texture().clone());

    gl_use_material(&post_processing);
    draw_texture_ex(
        render.stage.texture(),
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(1., 1.)),
            ..Default::default()
        },
    );
    gl_use_default_material();

    player_focus_draw(world, controls, time);
    render.post_processing.done_camera();
}
