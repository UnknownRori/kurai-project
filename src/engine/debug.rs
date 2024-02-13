use macroquad::{
    input::{is_key_down, is_mouse_button_down, mouse_position},
    math::vec2,
    miniquad::{KeyCode, MouseButton},
    ui::Ui,
};

use crate::{math::NormalizationVector2, window::Window};

use super::ui::TabbedWidgetBuilder;

#[derive(Debug, Default)]
pub struct Debugger {
    debug_mode: bool,
}

impl<'a> Debugger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, window: &Window) {
        if is_key_down(KeyCode::F1) {
            self.debug_mode = true;
        }
    }

    pub fn draw(&mut self, window: &'a Window) {
        if self.debug_mode {
            let screen = window.screen().clone();
            let playable_window = window.playable_window().clone();
            let game_window = window.game_window().clone();

            // TODO : Make this flexible
            TabbedWidgetBuilder::new(2, String::from("Debug"), vec2(400., 300.))
                .add(String::from("Debugger"), |ui: &mut Ui| {
                    ui.label(vec2(50., 50.), "Hello, World!");
                })
                .add(String::from("Entity"), |ui: &mut Ui| {
                    ui.label(vec2(50., 50.), "Entity");
                })
                .add(String::from("Window"), move |ui: &mut Ui| {
                    ui.label(None, "Window Area");
                    ui.separator();

                    ui.label(None, format!("Screen : {}", screen).as_str());
                    ui.label(
                        None,
                        format!("Game Window : {}", game_window.size()).as_str(),
                    );
                    ui.label(
                        None,
                        format!("Playarea : {}", playable_window.size()).as_str(),
                    );

                    ui.separator();
                    ui.same_line(0.45);
                    ui.label(None, "Mouse Position");
                    ui.separator();

                    // TODO : Make this to other function
                    let mouse_pos = mouse_position();
                    let normalized_playarea = (vec2(mouse_pos.0, mouse_pos.1)
                        - *playable_window.get_start())
                    .normalize_from_vec2(*playable_window.size());

                    ui.label(None, format!("Playarea : {}", normalized_playarea).as_str());
                })
                .location(vec2(20., 20.))
                .build()
                .draw();
        }
    }
}

// INFO : References
