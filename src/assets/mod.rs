pub mod konst;
mod preload;

pub use preload::preload;

use std::{collections::HashMap, error::Error};

use macroquad::{
    audio::{load_sound, Sound},
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct Assets {
    pub textures: HashMap<String, Texture2D>,
    pub materials: HashMap<String, Material>,
    pub audio: HashMap<String, Sound>,
    pub font: Font,
}

impl Assets {
    pub async fn new() -> Self {
        let mut font = load_ttf_font("./resources/fonts/PressStart2P-Regular.ttf")
            .await
            .unwrap();

        font.set_filter(FilterMode::Nearest);

        Self {
            textures: Default::default(),
            materials: Default::default(),
            audio: Default::default(),
            font,
        }
    }

    pub async fn load_audio(&mut self, path: &str, name: &str) -> Result<(), Box<dyn Error>> {
        if self.audio.contains_key(name) {
            return Ok(());
        }

        let sound = load_sound(path).await?;
        self.audio.insert(name.to_owned(), sound);
        Ok(())
    }

    pub async fn load_texture(
        &mut self,
        path: &str,
        name: &str,
        filter: Option<FilterMode>,
    ) -> Result<(), Box<dyn Error>> {
        if self.textures.contains_key(name) {
            return Ok(());
        }

        let texture = load_texture(path).await?;
        texture.set_filter(filter.unwrap_or(FilterMode::Nearest));
        self.textures.insert(name.to_owned(), texture);
        Ok(())
    }

    pub async fn load_material(
        &mut self,
        name: &str,
        vertex_path: &str,
        fragment_path: &str,
        params: MaterialParams,
    ) -> Result<(), Box<dyn Error>> {
        if self.materials.contains_key(name) {
            return Ok(());
        }

        let vertex = String::from_utf8(load_file(vertex_path).await?)?;
        let fragment = String::from_utf8(load_file(fragment_path).await?)?;
        let material = load_material(
            ShaderSource::Glsl {
                vertex: &vertex,
                fragment: &fragment,
            },
            params,
        )?;

        self.materials.insert(name.to_owned(), material);
        Ok(())
    }
}
