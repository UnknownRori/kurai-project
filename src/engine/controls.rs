use std::{collections::HashMap, hash::Hash};

use macroquad::{
    input::{is_key_down, is_key_pressed},
    miniquad::{KeyCode, KeyMods},
};

pub enum Combination {
    Single(KeyCode),
    Double(KeyCode, KeyMods),
}

pub struct Controls<T>(HashMap<T, Combination>)
where
    T: Hash + Eq + PartialEq;

impl<T> Controls<T>
where
    T: Hash + Eq + PartialEq,
{
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, action: T, combination: Combination) {
        self.0.insert(action, combination);
    }

    pub fn is_pressed(&self, action: T) -> bool {
        self.0.get(&action).map_or(false, |a| match a {
            Combination::Single(key) => is_key_pressed(*key),
            Combination::Double(key, mods) => {
                let mut combination_press = KeyMods::default();
                let key_press = is_key_pressed(*key);

                // TODO : Probably make this more flexible
                if mods.alt {
                    if is_key_pressed(KeyCode::LeftAlt) || is_key_pressed(KeyCode::RightAlt) {
                        combination_press.alt = true;
                    }
                }

                if mods.shift {
                    if is_key_pressed(KeyCode::LeftShift) || is_key_pressed(KeyCode::RightShift) {
                        combination_press.shift = true;
                    }
                }

                if mods.ctrl {
                    if is_key_pressed(KeyCode::LeftControl) || is_key_pressed(KeyCode::LeftControl)
                    {
                        combination_press.ctrl = true;
                    }
                }

                let expected_combination = vec![mods.alt, mods.shift, mods.ctrl];
                let result_combination = vec![
                    combination_press.alt,
                    combination_press.shift,
                    combination_press.ctrl,
                ];
                let combination_status = expected_combination
                    .iter()
                    .zip(result_combination.iter())
                    .all(|(&a, &b)| a == b);

                key_press && combination_status
            }
        })
    }

    pub fn is_key_down(&self, action: T) -> bool {
        self.0.get(&action).map_or(false, |a| match a {
            Combination::Single(key) => is_key_down(*key),
            Combination::Double(key, mods) => {
                let mut combination_press = KeyMods::default();
                let key_press = is_key_down(*key);

                // TODO : Probably make this more flexible
                if mods.alt {
                    if is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt) {
                        combination_press.alt = true;
                    }
                }

                if mods.shift {
                    if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                        combination_press.shift = true;
                    }
                }

                if mods.ctrl {
                    if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::LeftControl) {
                        combination_press.ctrl = true;
                    }
                }

                let expected_combination = vec![mods.alt, mods.shift, mods.ctrl];
                let result_combination = vec![
                    combination_press.alt,
                    combination_press.shift,
                    combination_press.ctrl,
                ];
                let combination_status = expected_combination
                    .iter()
                    .zip(result_combination.iter())
                    .all(|(&a, &b)| a == b);

                key_press && combination_status
            }
        })
    }
}
