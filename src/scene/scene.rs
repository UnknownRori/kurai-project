use hecs::World;

use crate::render::RenderingBuffer;

pub trait Scene {
    fn start(&mut self, _time: f64, _delta: f32);
    fn update(&mut self, _: &mut World, _time: f64, _delta: f32);
    fn draw(&self, _render: &RenderingBuffer, _time: f64, _delta: f32);
    fn end(&mut self);
}
