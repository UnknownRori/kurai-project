use macroquad::prelude::*;

use crate::window::Window;

const SHADOW_VERTEX: &'static str = include_str!("../../resources/shaders/shadow_vertex.glsl");
const SHADOW_FRAGMENT: &'static str = include_str!("../../resources/shaders/shadow_fragment.glsl");

// TODO : It's stupid and it's need clean up
pub fn shadow_shader_material(screen: &Window) -> Result<Material, color_eyre::Report> {
    let material = load_material(
        ShaderSource::Glsl {
            vertex: SHADOW_VERTEX,
            fragment: SHADOW_FRAGMENT,
        },
        MaterialParams {
            uniforms: vec![("screen_size".to_string(), UniformType::Float2)],
            ..Default::default()
        },
    )?;

    let screen_size = screen.screen();
    material.set_uniform("screen_size", screen_size);

    Ok(material)
}
