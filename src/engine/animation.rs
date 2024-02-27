use std::sync::Arc;

use macroquad::prelude::*;
use num_traits::ToPrimitive;

pub struct BackgroundAtlasAnimation {
    pub texture: Arc<Texture2D>,
    pub source_rect: Rect,
    pub tile: Vec2,
    pub time: f32,
    pub frame: usize,     // Total frame on atlas
    pub col_count: usize, // Max frame per width
    pub padding: f32,
    pub fps: f32,
    pub iteration: usize,
    pub is_playing: bool,
}

impl BackgroundAtlasAnimation {
    pub fn new(
        texture: &Arc<Texture2D>,
        col_count: usize,
        tile: Vec2,
        frame: usize,
        fps: f32,
        padding: f32,
        is_playing: bool,
    ) -> Self {
        Self {
            texture: Arc::clone(texture),
            col_count,
            tile,
            frame: frame - 1,
            fps,
            time: 0.,
            source_rect: Rect::new(0., 0., tile.x, tile.y),
            padding,
            iteration: 0,
            is_playing,
        }
    }

    pub fn update(&mut self, _: f64, delta: f32) {
        if self.is_playing {
            self.time += delta;
            if self.time > 1. / self.fps as f32 {
                self.time = 0.;

                let row = self.iteration / self.col_count;
                let col = self.iteration % self.col_count;
                self.source_rect = Rect::new(
                    if col == 0 {
                        col as f32 * self.tile.x
                    } else {
                        col as f32 * self.tile.x + col as f32 * self.padding
                    },
                    if row == 0 {
                        row as f32 * self.tile.y
                    } else {
                        row as f32 * self.tile.y + row as f32 * self.padding
                    },
                    self.tile.x,
                    self.tile.y,
                );

                self.iteration += 1;
                if self.iteration > self.frame {
                    self.iteration = 0;
                }
            }
        }
    }

    pub const fn source_rect(&self) -> &Rect {
        &self.source_rect
    }
}
