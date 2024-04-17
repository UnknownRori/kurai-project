use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use hecs::World;

use crate::components::transform2d::Transform2D;

pub type AttackSpawnFn = Arc<Mutex<dyn AttackSpawner>>;

pub trait AttackSpawner: Send + Sync + Debug {
    fn spawn(
        &mut self,
        _world: &mut World,
        _current: &Transform2D,
        _player: &Transform2D,
        _delta: f32,
    );
}

#[derive(Debug, Clone)]
pub struct PlayerAttack {
    pub normal: AttackInfo,
    pub focus: AttackInfo,
    pub spell: SpellInfo,
}

#[derive(Debug, Clone)]
pub enum Attack {
    Attack(AttackInfo),
    Focus(AttackInfo),
    Spell(SpellInfo),
}

#[derive(Debug, Clone)]
pub struct AttackInfo {
    // TODO : Is it really need to be a Arc<Mutex<T>>(?)
    pub spawner: AttackSpawnFn,
}

impl AttackInfo {
    pub fn new(spawner: AttackSpawnFn) -> Self {
        Self { spawner }
    }
}

#[derive(Debug, Clone)]
pub struct SpellInfo {
    pub id: usize, // TODO : Maybe for replay purpose
    pub name: String,
    pub timeout: f64,
    pub action: AttackSpawnFn,
}

impl SpellInfo {
    pub fn new(id: usize, name: String, timeout: f64, action: AttackSpawnFn) -> Self {
        Self {
            id,
            name,
            timeout,
            action,
        }
    }
}
