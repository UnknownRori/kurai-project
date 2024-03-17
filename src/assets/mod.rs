use macroquad::miniquad::FilterMode;

use crate::engine::assets::AssetsManager;

pub mod konst;
pub mod preload;

pub async fn preload_texture(assets_manager: &mut AssetsManager, name: &str, path: &str) {
    assets_manager
        .textures
        .register(name, path, Some(FilterMode::Nearest))
        .await
        .unwrap();
}

pub async fn preload_sfx(assets_manager: &mut AssetsManager, name: &str, path: &str) {
    assets_manager.sfx.register(name, path).await.unwrap();
}
