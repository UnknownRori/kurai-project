use std::{collections::HashMap, rc::Rc};

use color_eyre::eyre::{eyre, Ok};
use macroquad::{
    audio::{load_sound, Sound},
    texture::{load_texture, Texture2D},
};

#[derive(Default)]
pub struct AssetsManager {
    texture_collection: HashMap<String, Rc<Texture2D>>,
    sfx_collection: HashMap<String, Rc<Sound>>,
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
            None => Err(eyre!("Cannot store texture with the same name!")),
            _ => Ok(()),
        }
    }

    pub async fn register_sfx(
        &mut self,
        name: &str,
        file_name: &str,
    ) -> Result<(), color_eyre::Report> {
        let sfx = load_sound(file_name).await?;

        match self.sfx_collection.insert(name.to_owned(), sfx.into()) {
            None => Err(eyre!("Cannot store sfx with the same name!")),
            _ => Ok(()),
        }
    }

    pub fn get_texture(&self, name: &str) -> Option<Rc<Texture2D>> {
        self.texture_collection.get(name).map(|a| Rc::clone(a))
    }

    pub fn get_sfx(&self, name: &str) -> Option<Rc<Sound>> {
        self.sfx_collection.get(name).map(|a| Rc::clone(a))
    }
}
