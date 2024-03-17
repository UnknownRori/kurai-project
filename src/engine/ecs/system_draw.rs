use hecs::World;

pub struct SystemDraw(Vec<Box<dyn Fn(&World) -> () + 'static>>);

impl SystemDraw {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add_system(&mut self, system: impl Fn(&World) -> () + 'static) {
        self.0.push(Box::new(system));
    }

    pub fn draw(&self, world: &World) {
        for system in &self.0 {
            (*system)(world);
        }
    }
}
