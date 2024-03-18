use std::{collections::HashMap, error::Error, sync::Arc};

use macroquad::{
    audio::{load_sound, Sound},
    file::load_file,
    material::{load_material, Material, MaterialParams},
    miniquad::{FilterMode, ShaderSource},
    texture::{load_texture, Texture2D},
};

pub type SharedAsset<T> = Arc<T>;

#[derive(Default, Debug)]
pub struct AssetsManager {
    pub textures: TextureHandler,
    pub sfx: SfxHandler,
    pub shaders: ShaderHandler,
    pub bgm: BgmHandler,
}

#[derive(Default, Debug)]
pub struct TextureHandler(HashMap<String, SharedAsset<Texture2D>>);

impl TextureHandler {
    pub async fn register(
        &mut self,
        name: &str,
        path: &str,
        filter: Option<FilterMode>,
    ) -> Result<(), Box<dyn Error>> {
        if self.0.contains_key(name) {
            return Ok(());
        }

        let texture = load_texture(path).await?;
        texture.set_filter(filter.unwrap_or(FilterMode::Nearest));
        self.0.insert(name.to_owned(), Arc::new(texture));

        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<Arc<Texture2D>> {
        self.0.get(name).map(|a| Arc::clone(&a))
    }
}

#[derive(Default, Debug)]
pub struct SfxHandler(HashMap<String, SharedAsset<Sound>>);

impl SfxHandler {
    pub async fn register(&mut self, name: &str, path: &str) -> Result<(), Box<dyn Error>> {
        if self.0.contains_key(name) {
            return Ok(());
        }

        let sound = load_sound(path).await?;
        self.0.insert(name.to_owned(), Arc::new(sound));
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<Arc<Sound>> {
        self.0.get(name).map(|a| Arc::clone(&a))
    }
}

#[derive(Default, Debug)]
pub struct ShaderHandler(HashMap<String, SharedAsset<Material>>);

impl ShaderHandler {
    pub async fn register(
        &mut self,
        name: &str,
        vertex_path: &str,
        fragment_path: &str,
        params: MaterialParams,
    ) -> Result<(), Box<dyn Error>> {
        if self.0.contains_key(name) {
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

        self.0.insert(name.to_owned(), Arc::new(material));
        Ok(())
    }

    pub async fn batch(
        &mut self,
        batch: Vec<(&str, &str, &str, MaterialParams)>,
    ) -> Result<(), Box<dyn Error>> {
        for (name, vertex, fragment, params) in batch {
            self.register(name, vertex, fragment, params).await?;
        }

        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<Arc<Material>> {
        self.0.get(name).map(|a| Arc::clone(&a))
    }
}

#[derive(Default, Debug)]
pub struct BgmHandler(HashMap<String, SharedAsset<Sound>>);

impl BgmHandler {
    pub async fn register(&mut self, name: &str, path: &str) -> Result<(), Box<dyn Error>> {
        if self.0.contains_key(name) {
            return Ok(());
        }

        let sound = load_sound(path).await?;
        self.0.insert(name.to_owned(), Arc::new(sound));
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<Arc<Sound>> {
        self.0.get(name).map(|a| Arc::clone(&a))
    }
}
