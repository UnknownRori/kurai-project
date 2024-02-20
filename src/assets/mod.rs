use std::{collections::HashMap, sync::Arc};

use color_eyre::eyre::Ok;
use macroquad::{
    audio::{load_sound, Sound},
    file::load_file,
    material::{load_material, Material, MaterialParams},
    miniquad::{FilterMode, ShaderSource},
    texture::{load_texture, Texture2D},
};

pub trait AssetsHandler {
    type Output;

    async fn register(&mut self, name: &str, path: &str) -> Result<(), color_eyre::Report>;
    async fn batch(&mut self, batch: &[(&str, &str)]) -> Result<(), color_eyre::Report> {
        for (name, path) in batch {
            self.register(name, path).await?;
        }

        Ok(())
    }
    fn get(&self, name: &str) -> Option<Self::Output>;
}

#[derive(Default)]
pub struct TextureHandler(HashMap<String, Arc<Texture2D>>);
#[derive(Default)]
pub struct SfxHandler(HashMap<String, Arc<Sound>>);
#[derive(Default)]
pub struct ShaderHandler(HashMap<String, Arc<Material>>);

impl AssetsHandler for TextureHandler {
    type Output = Arc<Texture2D>;

    async fn register(&mut self, name: &str, path: &str) -> Result<(), color_eyre::Report> {
        let texture = load_texture(path).await?;
        texture.set_filter(FilterMode::Nearest);
        self.0.insert(name.to_owned(), Arc::new(texture));
        Ok(())
    }

    fn get(&self, name: &str) -> Option<Self::Output> {
        self.0.get(name).map(|a| Arc::clone(&a))
    }
}

impl AssetsHandler for SfxHandler {
    type Output = Arc<Sound>;

    async fn register(&mut self, name: &str, path: &str) -> Result<(), color_eyre::Report> {
        let sound = load_sound(path).await?;
        self.0.insert(name.to_owned(), Arc::new(sound));
        Ok(())
    }

    fn get(&self, name: &str) -> Option<Self::Output> {
        self.0.get(name).map(|a| Arc::clone(&a))
    }
}

impl ShaderHandler {
    pub async fn register(
        &mut self,
        name: &str,
        vertex_path: &str,
        fragment_path: &str,
        params: MaterialParams,
    ) -> Result<(), color_eyre::Report> {
        let vertex = String::from_utf8(load_file(vertex_path).await?)?;
        let fragment = String::from_utf8(load_file(fragment_path).await?)?;
        let material = load_material(
            ShaderSource::Glsl {
                vertex: &vertex,
                fragment: &fragment,
            },
            params,
        )?;

        self.0.insert(name.to_owned(), Arc::new(material));
        Ok(())
    }

    pub async fn batch(
        &mut self,
        batch: Vec<(&str, &str, &str, MaterialParams)>,
    ) -> Result<(), color_eyre::Report> {
        for (name, vertex, fragment, params) in batch {
            self.register(name, vertex, fragment, params).await?;
        }

        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<Arc<Material>> {
        self.0.get(name).map(|a| Arc::clone(&a))
    }
}

#[derive(Default)]
pub struct AssetsManager {
    pub textures: TextureHandler,
    pub sfx: SfxHandler,
    pub shaders: ShaderHandler,
}
