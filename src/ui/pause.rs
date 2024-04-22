use macroquad::prelude::*;

use crate::controls::{Action, Controls};

use super::draw_text_ex2;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum PauseChoice {
    #[default]
    Resume,
    Restart,
    Exit,
}

#[derive(Debug, Default)]
pub struct PauseUI {
    selected_choice: PauseChoice,
}

impl PauseUI {
    const INACTIVE: Color = GRAY;
    const ACTIVE: Color = WHITE;

    pub fn update(&mut self, controls: &Controls) -> Option<PauseChoice> {
        if controls.is_pressed(Action::Escape) {
            self.selected_choice = PauseChoice::Resume;
        }

        if controls.is_pressed(Action::Down) {
            self.selected_choice = match self.selected_choice {
                PauseChoice::Resume => PauseChoice::Restart,
                PauseChoice::Restart => PauseChoice::Exit,
                PauseChoice::Exit => PauseChoice::Resume,
            };
        }

        if controls.is_pressed(Action::Up) {
            self.selected_choice = match self.selected_choice {
                PauseChoice::Resume => PauseChoice::Exit,
                PauseChoice::Restart => PauseChoice::Resume,
                PauseChoice::Exit => PauseChoice::Restart,
            };
        }

        if controls.is_pressed(Action::Accept) {
            return Some(self.selected_choice.clone());
        }

        return None;
    }

    pub fn draw(&self, font: &Font) {
        draw_rectangle(0., 0., 1., 1., Color::new(0., 0., 0., 0.75));

        draw_text_ex2(
            "Game Paused",
            0.605,
            0.36,
            0.05,
            Self::ACTIVE,
            true,
            Some(font),
        );

        if self.selected_choice == PauseChoice::Resume {
            draw_text_ex2("Resume", 0.54, 0.45, 0.035, Self::ACTIVE, true, Some(font));
        } else {
            draw_text_ex2(
                "Resume",
                0.54,
                0.45,
                0.035,
                Self::INACTIVE,
                true,
                Some(font),
            );
        }

        if self.selected_choice == PauseChoice::Restart {
            draw_text_ex2("Restart", 0.55, 0.50, 0.035, Self::ACTIVE, true, Some(font));
        } else {
            draw_text_ex2(
                "Restart",
                0.55,
                0.50,
                0.035,
                Self::INACTIVE,
                true,
                Some(font),
            );
        }

        if self.selected_choice == PauseChoice::Exit {
            draw_text_ex2("Exit", 0.53, 0.55, 0.035, Self::ACTIVE, true, Some(font));
        } else {
            draw_text_ex2("Exit", 0.53, 0.55, 0.035, Self::INACTIVE, true, Some(font));
        }
    }
}
