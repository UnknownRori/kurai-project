use crate::components::{Controllable, Movable, Player};

pub type PlayerEntity<'a> = (&'a Player, &'a Controllable, &'a Movable);
