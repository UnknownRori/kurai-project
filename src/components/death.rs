use crate::{konst::DONE_BLINKING, time::Timer};

pub struct Death;

#[derive(Debug, Clone)]
pub struct DeathBlinkingAnimation(pub Timer);

impl Default for DeathBlinkingAnimation {
    fn default() -> Self {
        Self(Timer::new(1., true))
    }
}

impl DeathBlinkingAnimation {
    pub fn update(&mut self) {
        if self.0.completed() {
            self.0.time = self.0.time * 0.8;
            self.0.reset();
            return;
        }
        self.0.update();
    }

    pub fn done(&self) -> bool {
        self.0.time <= DONE_BLINKING
    }

    pub fn should_blink(&self) -> bool {
        !self.0.completed()
    }
}
