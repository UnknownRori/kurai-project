use std::sync::{Arc, Mutex};

use hecs::World;

use super::Transform2D;

pub trait AttackSpawner: Sync + Send {
    fn spawn(&mut self, _: &mut World, _current: &Transform2D, _player: Option<&Transform2D>);
}

#[derive(Clone)]
pub struct PlayerAttackInfo {
    pub normal: AttackInfo,
    pub focus: AttackInfo,
    pub spell: SpellInfo,
}

#[derive(Clone)]
pub struct AttackInfo {
    pub spawn: Arc<Mutex<dyn AttackSpawner>>,
}

impl AttackInfo {
    pub fn new(spawn: Arc<Mutex<dyn AttackSpawner>>) -> Self {
        Self { spawn }
    }
}

#[derive(Clone)]
pub struct SpellInfo {
    pub name: String,
    pub timeout: f32,
    pub spawn: Arc<Mutex<dyn AttackSpawner>>,
}

impl SpellInfo {
    pub fn new(name: &str, timeout: f32, spawn: Arc<Mutex<dyn AttackSpawner>>) -> Self {
        Self {
            name: name.to_owned(),
            timeout,
            spawn,
        }
    }
}
