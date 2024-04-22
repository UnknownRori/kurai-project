use crate::controls::{Action, Controls};
use macroquad::prelude::*;

use super::draw_text_ex2;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum GameOverChoice {
    #[default]
    Continue,
    Restart,
    Exit,
}

#[derive(Debug, Default)]
pub struct GameOverUI {
    selected_choice: GameOverChoice,
}

impl GameOverUI {
    const INACTIVE: Color = GRAY;
    const ACTIVE: Color = WHITE;

    pub fn update(&mut self, controls: &Controls) -> Option<GameOverChoice> {
        if controls.is_pressed(Action::Escape) {
            self.selected_choice = GameOverChoice::Continue;
        }

        if controls.is_pressed(Action::Down) {
            self.selected_choice = match self.selected_choice {
                GameOverChoice::Continue => GameOverChoice::Restart,
                GameOverChoice::Restart => GameOverChoice::Exit,
                GameOverChoice::Exit => GameOverChoice::Continue,
            };
        }

        if controls.is_pressed(Action::Up) {
            self.selected_choice = match self.selected_choice {
                GameOverChoice::Continue => GameOverChoice::Exit,
                GameOverChoice::Restart => GameOverChoice::Continue,
                GameOverChoice::Exit => GameOverChoice::Restart,
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
            "Game Over",
            0.605,
            0.36,
            0.05,
            Self::ACTIVE,
            true,
            Some(font),
        );

        if self.selected_choice == GameOverChoice::Continue {
            draw_text_ex2(
                "Continue (-)",
                0.54,
                0.45,
                0.035,
                Self::ACTIVE,
                true,
                Some(font),
            );
        } else {
            draw_text_ex2(
                "Continue (-)",
                0.54,
                0.45,
                0.035,
                Self::INACTIVE,
                true,
                Some(font),
            );
        }

        if self.selected_choice == GameOverChoice::Restart {
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

        if self.selected_choice == GameOverChoice::Exit {
            draw_text_ex2("Exit", 0.53, 0.55, 0.035, Self::ACTIVE, true, Some(font));
        } else {
            draw_text_ex2("Exit", 0.53, 0.55, 0.035, Self::INACTIVE, true, Some(font));
        }
    }
}
