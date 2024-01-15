use std::collections::HashMap;

use macroquad::input::KeyCode;

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

pub struct Controls {
    controls: HashMap<Action, KeyCode>,
}

impl Default for Controls {
    fn default() -> Self {
        let mut controls = HashMap::with_capacity(8);

        controls.insert(Action::Attack, KeyCode::Z);
        controls.insert(Action::Spell, KeyCode::C);
        controls.insert(Action::Focus, KeyCode::LeftShift);

        controls.insert(Action::Up, KeyCode::Up);
        controls.insert(Action::Down, KeyCode::Down);
        controls.insert(Action::Left, KeyCode::Left);
        controls.insert(Action::Right, KeyCode::Right);

        controls.insert(Action::Escape, KeyCode::Escape);

        Self { controls }
    }
}
