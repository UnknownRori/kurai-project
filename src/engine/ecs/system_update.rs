use hecs::World;

pub struct SystemUpdate(Vec<Box<dyn Fn(&mut World) -> () + 'static>>);

impl SystemUpdate {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add_system(&mut self, system: impl Fn(&mut World) -> () + 'static) {
        self.0.push(Box::new(system));
    }

    pub fn update(&self, world: &mut World) {
        for system in &self.0 {
            (*system)(world);
        }
    }
}
