use crate::controls::{Action, Controls};

pub struct Controllable<'a>(pub &'a Controls<Action>);
