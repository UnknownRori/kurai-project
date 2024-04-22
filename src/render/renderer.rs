use macroquad::prelude::*;

use crate::{
    assets::{
        konst::{BLOOM, HUD, LIGHTMAP, POST_PROCESSING},
        Assets,
    },
    konst::{
        DESIRED_ASPECT_RATIO, GAME_VERSION, VIRTUAL_SCREEN_WIDTH, VIRTUAL_STAGE_ASPECT_RATIO,
        VIRTUAL_STAGE_HEIGHT, VIRTUAL_STAGE_WIDTH,
    },
    scene::Scene,
    system::{draw_entity, draw_focus},
    utils::{get_adjusted_screen, FPSCounter},
    vec2, Game,
};

use super::{create_camera2d, render_target::create_render_target_from_aspect_ratio};

pub struct Renderer {
    pub entities: Camera2D,
    // pub lightmap: Camera2D,
    // pub bloom: Camera2D,
    // pub post_processing: Camera2D,
    pub stage: Camera2D,
    pub ui: Camera2D,
}

impl Default for Renderer {
    fn default() -> Self {
        let coordinate = Rect::new(0., 0., 1., 1.);

        // INFO : entities
        let render_target_entities = render_target(VIRTUAL_STAGE_WIDTH, VIRTUAL_STAGE_HEIGHT);
        render_target_entities
            .texture
            .set_filter(FilterMode::Nearest);
        let entities = create_camera2d(coordinate.clone(), render_target_entities);

        // // INFO : Lightmap
        // let render_target_lightmap = render_target(VIRTUAL_STAGE_WIDTH, VIRTUAL_STAGE_HEIGHT);
        // render_target_lightmap
        //     .texture
        //     .set_filter(FilterMode::Nearest);
        // let lightmap = create_camera2d(coordinate.clone(), render_target_lightmap);
        // //
        // // INFO : Bloom
        // let render_target_bloom = render_target(VIRTUAL_STAGE_WIDTH, VIRTUAL_STAGE_HEIGHT);
        // render_target_bloom.texture.set_filter(FilterMode::Nearest);
        // let bloom = create_camera2d(coordinate.clone(), render_target_bloom);
        //
        // // INFO : Post Processing
        // let render_target_post = render_target(VIRTUAL_STAGE_WIDTH, VIRTUAL_STAGE_HEIGHT);
        // render_target_post.texture.set_filter(FilterMode::Nearest);
        // let post_processing = create_camera2d(coordinate.clone(), render_target_post);

        // INFO : Stage
        let render_target_stage = render_target(VIRTUAL_STAGE_WIDTH, VIRTUAL_STAGE_HEIGHT);
        render_target_stage.texture.set_filter(FilterMode::Nearest);
        let stage = create_camera2d(coordinate.clone(), render_target_stage);

        // INFO : UI
        let render_target_ui = create_render_target_from_aspect_ratio(
            VIRTUAL_SCREEN_WIDTH,
            VIRTUAL_STAGE_ASPECT_RATIO,
        );
        render_target_ui.texture.set_filter(FilterMode::Nearest);
        let ui = create_camera2d(coordinate.clone(), render_target_ui);

        Self {
            entities,
            // lightmap,
            // bloom,
            // post_processing,
            stage,
            ui,
        }
    }
}

pub fn compose(game: &Game) {
    let renderer = &game.renderer;
    let fps = &game.fps;
    let assets = &game.assets;

    match game.scene {
        Scene::MainMenu => todo!(),
        Scene::Options => todo!(),
        Scene::Stage => {
            let stage = game
                .current_stage
                .as_ref()
                .expect("Must already set the stage!");

            // INFO : Entities
            set_camera(&renderer.entities);
            clear_background(Color::new(0., 0., 0., 0.));
            draw_entity(&game.world);
            set_default_camera();

            // INFO : Stage
            set_camera(&renderer.stage);
            clear_background(Color::new(0., 0., 0., 0.));
            stage.draw(game);
            draw_texture_ex(
                &renderer.entities.render_target.as_ref().unwrap().texture,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2!(1.)),
                    ..Default::default()
                },
            );

            draw_focus(&game.world, &assets, &game.controls);

            match game.state {
                crate::state::GameState::Running => {}
                crate::state::GameState::Pause => game.pause_ui.draw(&assets.font),
                crate::state::GameState::GameOver => game.game_over_ui.draw(&assets.font),
            }

            set_default_camera();

            // fetch_lightmap(assets, &renderer.stage, &renderer.lightmap);
            // generate_bloom(assets, &renderer.lightmap, &renderer.bloom);
            //
            // // INFO : Post processing
            // set_camera(&renderer.post_processing);
            // clear_background(BLACK);
            // let texture = &renderer.stage.render_target.as_ref().unwrap().texture;
            // let material = assets.materials.get(POST_PROCESSING).unwrap();
            // material.set_uniform("iResolution", texture.size());
            // material.set_uniform("exposure", 0.8f32);
            // material.set_texture(
            //     "bloom",
            //     renderer
            //         .bloom
            //         .render_target
            //         .as_ref()
            //         .unwrap()
            //         .texture
            //         .clone(),
            // );
            // material.set_texture(
            //     "stage",
            //     renderer
            //         .stage
            //         .render_target
            //         .as_ref()
            //         .unwrap()
            //         .texture
            //         .clone(),
            // );
            // gl_use_material(material);
            // draw_texture_ex(
            //     texture,
            //     0.,
            //     0.,
            //     WHITE,
            //     DrawTextureParams {
            //         dest_size: Some(vec2(1., 1.)),
            //         ..Default::default()
            //     },
            // );
            // gl_use_default_material();
            // set_default_camera();

            // INFO : UI
            set_camera(&renderer.ui);
            clear_background(BLACK);
            let stage = &renderer.stage.render_target.as_ref().unwrap().texture;
            draw_texture_ex(
                stage,
                0.03,
                0.009,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(0.64, 0.975)),
                    ..Default::default()
                },
            );
            let hud = assets.textures.get(HUD).unwrap();
            draw_texture_ex(
                &hud,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2!(1.)),
                    ..Default::default()
                },
            );
            draw_score(&game);
            draw_hud_info(&assets.font, fps);
            draw_entity_number(game.world.len(), &assets.font);
            set_default_camera();

            // INFO : Draw the buffer to the screen
            clear_background(BLACK);
            let width = screen_width();
            let height = screen_height();
            let adjusted = get_adjusted_screen(DESIRED_ASPECT_RATIO);
            let offset = vec2((width - adjusted.x) / 2f32, (height - adjusted.y) / 2f32);
            let ui = &renderer.ui.render_target.as_ref().unwrap().texture;
            draw_texture_ex(
                ui,
                offset.x,
                offset.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(adjusted),
                    ..Default::default()
                },
            );
        }
    }
}

pub fn draw_hud_info(font: &Font, fps: &FPSCounter) {
    fps.draw(font);

    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(0.02);
    let len = measure_text(GAME_VERSION, Some(font), font_size, font_scale);
    draw_text_ex(
        GAME_VERSION,
        1. - len.width + 0.02,
        len.height + 0.01,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    )
}

fn draw_entity_number(len: u32, font: &Font) {
    // INFO : For debugging purposes
    let total_entity = format!("{}", len);
    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(0.02);
    let len = measure_text(&total_entity, Some(font), font_size, font_scale);
    draw_text_ex(
        &total_entity,
        0.0,
        0.0 + len.height + 0.01,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );
}

fn draw_score(game: &Game) {
    // TODO : Create immediate UI

    let score = &game.score;
    let font = &game.assets.font;
    const SCALE: f32 = 0.03;
    const SCALE_WIDTH: f32 = SCALE - 0.01;

    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    let len = measure_text("Score", Some(font), font_size, font_scale);
    draw_text_ex(
        "Score",
        0.8 - len.width + SCALE_WIDTH,
        0.1,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );

    let score_text = format!("{:08}", score.score);
    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    draw_text_ex(
        &score_text,
        0.8 + SCALE_WIDTH,
        0.1,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );

    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    let len = measure_text("Score", Some(font), font_size, font_scale);
    draw_text_ex(
        "Value",
        0.8 - len.width + SCALE_WIDTH,
        0.12 + len.height,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );

    let value = format!("{:08}", score.value);
    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    draw_text_ex(
        &value,
        0.8 + SCALE_WIDTH,
        0.12 + len.height,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );

    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    let len = measure_text("Score", Some(font), font_size, font_scale);
    draw_text_ex(
        "Graze",
        0.8 - len.width + SCALE_WIDTH,
        0.14 + len.height * 2.,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );

    let value = format!("{:04}", score.graze);
    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    draw_text_ex(
        &value,
        0.8 + SCALE_WIDTH,
        0.14 + len.height * 2.,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );

    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    let len = measure_text("Score", Some(font), font_size, font_scale);
    draw_text_ex(
        "Life",
        0.8 - len.width + SCALE_WIDTH,
        0.34 + len.height,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );

    let value = format!("{}", score.life);
    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    draw_text_ex(
        &value,
        0.8 + SCALE_WIDTH,
        0.34 + len.height,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );

    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    let len = measure_text("Score", Some(font), font_size, font_scale);
    draw_text_ex(
        "Power",
        0.8 - len.width + SCALE_WIDTH,
        0.38 + len.height,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );

    let value = format!("{:.2}", score.power);
    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    draw_text_ex(
        &value,
        0.8 + SCALE_WIDTH,
        0.38 + len.height,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );

    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    let len = measure_text("Score", Some(font), font_size, font_scale);
    draw_text_ex(
        "Spell",
        0.8 - len.width + SCALE_WIDTH,
        0.42 + len.height,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );

    let value = format!("{}", score.spell);
    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(SCALE);
    draw_text_ex(
        &value,
        0.8 + SCALE_WIDTH,
        0.42 + len.height,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    );
}
pub fn fetch_lightmap(assets: &Assets, from: &Camera2D, to: &Camera2D) {
    let texture = &from.render_target.as_ref().unwrap().texture;

    set_camera(to);
    clear_background(Color::new(0., 0., 0., 0.));

    let lightmap_shader = assets.materials.get(LIGHTMAP).unwrap();
    lightmap_shader.set_uniform("iResolution", texture.size());
    gl_use_material(&lightmap_shader);
    draw_texture_ex(
        texture,
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(1., 1.)),
            ..Default::default()
        },
    );
    gl_use_default_material();
    set_default_camera();
}

pub fn generate_bloom(assets: &Assets, from: &Camera2D, to: &Camera2D) {
    let texture = &from.render_target.as_ref().unwrap().texture;

    set_camera(to);
    clear_background(Color::new(0., 0., 0., 0.));
    let bloom_shader = assets.materials.get(BLOOM).unwrap();
    bloom_shader.set_uniform("iResolution", texture.size());
    gl_use_material(&bloom_shader);
    bloom_shader.set_uniform("horizontal", 1);
    draw_texture_ex(
        texture,
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(1., 1.)),
            ..Default::default()
        },
    );
    bloom_shader.set_uniform("horizontal", 0);
    draw_texture_ex(
        texture,
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(1., 1.)),
            ..Default::default()
        },
    );
    gl_use_default_material();
    set_default_camera();
}
