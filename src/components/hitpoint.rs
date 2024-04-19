use crate::time::Timer;

#[derive(Debug, Clone, Copy)]
pub struct Hitpoint {
    pub hp: f32,
    pub max_hp: f32,
    pub invulnerable: bool, // INFO : Phase for invulnerable stuff
}

#[derive(Debug, Clone, Copy)]
pub struct Damaged;

impl Hitpoint {
    pub fn new(hp: f32) -> Self {
        Self {
            hp,
            max_hp: hp,
            invulnerable: false,
        }
    }

    pub fn is_dead(&self) -> bool {
        if self.invulnerable {
            return false;
        }

        return self.hp < 0.;
    }

    pub fn damage(&mut self, damage: f32) -> bool {
        if !self.invulnerable {
            self.hp -= damage;
            return self.hp <= 0.0;
        }

        false
    }
}
