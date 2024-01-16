use crate::components::{Controllable, DummyDraw, Movable, Player, Position};

pub type PlayerEntity<'a> = (
    &'a Player,
    &'a Controllable,
    &'a Movable,
    &'a mut Position,
    &'a DummyDraw,
);
