use crate::time::Instant;

pub struct DemoStage {
    start: Instant,
}

impl DemoStage {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn run() {}
}
