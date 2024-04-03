use std::{fmt::Debug, sync::Arc};

use hecs::World;

use crate::engine::{components::Transform2D, time::Instant};

pub trait AttackSpawner: Send + Sync + Debug {
    fn spawn(
        &self,
        _world: &mut World,
        _current: &Transform2D,
        _player: &Transform2D,
        _bullet_speed: f32,
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
pub struct GenericAttackInfo {
    pub bullet_speed: f32,
    pub fire_rate: f64,
    pub last_shoot: Option<Instant>,
}

impl GenericAttackInfo {
    pub fn new(bullet_speed: f32, fire_rate: f64) -> Self {
        Self {
            bullet_speed,
            fire_rate,
            last_shoot: None,
        }
    }

    pub fn can_fire(&self, time_frame: f64) -> bool {
        if self.last_shoot.is_none() {
            return true;
        }

        self.last_shoot.unwrap().elapsed(time_frame) >= 1.0 / self.fire_rate
    }

    pub fn update_cooldown(&mut self) {
        self.last_shoot = Some(Instant::now());
    }
}

#[derive(Debug, Clone)]
pub enum Attack {
    Attack(AttackInfo),
    Focus(AttackInfo),
    Spell(SpellInfo),
}

#[derive(Debug, Clone)]
pub struct AttackInfo {
    pub spawner: Arc<dyn AttackSpawner>,
    pub info: GenericAttackInfo,
}

impl AttackInfo {
    pub fn new(info: GenericAttackInfo, spawner: Arc<dyn AttackSpawner>) -> Self {
        Self { info, spawner }
    }
}

#[derive(Debug, Clone)]
pub struct SpellInfo {
    pub id: usize, // TODO : Maybe for replay purpose
    pub name: String,
    pub timeout: f64,
    pub action: Arc<dyn AttackSpawner>,

    pub info: GenericAttackInfo,
}

impl SpellInfo {
    pub fn new(
        id: usize,
        name: String,
        timeout: f64,
        action: Arc<dyn AttackSpawner>,
        info: GenericAttackInfo,
    ) -> Self {
        Self {
            id,
            name,
            timeout,
            info,
            action,
        }
    }
}
