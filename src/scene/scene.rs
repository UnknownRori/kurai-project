use hecs::World;

pub trait Scene {
    fn start(&mut self, _time: f64, _delta: f32);
    fn update(&mut self, _: &mut World, _time: f64, _delta: f32);
    fn draw(&self, _time: f64, _delta: f32);
    fn end(&mut self);
}
