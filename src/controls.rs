use macroquad::miniquad::KeyCode;

use crate::engine::controls::{Combination, Controls};

#[derive(Hash, Eq, PartialEq)]
pub enum Action {
    Attack,
    Spell,
    Focus,

    Up,
    Right,
    Left,
    Down,

    Escape,
}

pub fn init_controls() -> Controls<Action> {
    let mut controls = Controls::new();

    controls.add(Action::Attack, Combination::Single(KeyCode::Z));
    controls.add(Action::Focus, Combination::Single(KeyCode::LeftShift));
    controls.add(Action::Spell, Combination::Single(KeyCode::X));

    controls.add(Action::Up, Combination::Single(KeyCode::Up));
    controls.add(Action::Left, Combination::Single(KeyCode::Left));
    controls.add(Action::Right, Combination::Single(KeyCode::Right));
    controls.add(Action::Down, Combination::Single(KeyCode::Down));

    controls.add(Action::Escape, Combination::Single(KeyCode::Escape));

    controls
}
