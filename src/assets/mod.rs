use std::{collections::HashMap, sync::Arc};

use color_eyre::eyre::{eyre, Ok};
use macroquad::{
    audio::{load_sound, Sound},
    texture::{load_texture, Texture2D},
};

#[derive(Default)]
pub struct AssetsManager {
    texture_collection: HashMap<String, Arc<Texture2D>>,
    sfx_collection: HashMap<String, Arc<Sound>>,
}

impl AssetsManager {
    pub async fn register_texture(
        &mut self,
        name: &str,
        file_name: &str,
    ) -> Result<(), color_eyre::Report> {
        let texture = load_texture(file_name).await?;

        match self
            .texture_collection
            .insert(name.to_owned(), texture.into())
        {
            Some(_) => Err(eyre!("Cannot store texture with the same name!")),
            None => Ok(()),
        }
    }

    pub async fn register_sfx(
        &mut self,
        name: &str,
        file_name: &str,
    ) -> Result<(), color_eyre::Report> {
        let sfx = load_sound(file_name).await?;

        match self.sfx_collection.insert(name.to_owned(), sfx.into()) {
            Some(_) => Err(eyre!("Cannot store sfx with the same name!")),
            None => Ok(()),
        }
    }

    pub async fn register_texture_batch(
        &mut self,
        batch: &[(&str, &str)],
    ) -> Result<(), color_eyre::Report> {
        for a in batch {
            self.register_texture(a.0, a.1).await?;
        }
        Ok(())
    }

    pub async fn register_sfx_batch(
        &mut self,
        batch: &[(&str, &str)],
    ) -> Result<(), color_eyre::Report> {
        for a in batch {
            self.register_sfx(a.0, a.1).await?;
        }
        Ok(())
    }

    pub fn get_texture(&self, name: &str) -> Option<Arc<Texture2D>> {
        self.texture_collection.get(name).map(|a| Arc::clone(a))
    }

    pub fn get_sfx(&self, name: &str) -> Option<Arc<Sound>> {
        self.sfx_collection.get(name).map(|a| Arc::clone(a))
    }
}
