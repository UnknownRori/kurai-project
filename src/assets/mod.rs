use std::rc::Rc;

use macroquad::texture::Texture2D;

#[derive(Default)]
pub struct AssetsManager {
    buffer: Vec<Rc<Texture2D>>,
}
