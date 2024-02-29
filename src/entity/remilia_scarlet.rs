use crate::components::{AttackInfo, Controllable, Movable, Player, Position};

pub struct RemiliaScarlet;

pub type RemiliaScarletEntity<'a> = (
    &'a Player,
    &'a Controllable,
    &'a Movable,
    &'a mut Position,
    &'a AttackInfo,
    &'a RemiliaScarlet,
);
