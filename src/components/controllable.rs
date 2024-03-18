use crate::{controls::Action, engine::controls::Controls};

pub struct Controllable<'a>(pub &'a Controls<Action>);
