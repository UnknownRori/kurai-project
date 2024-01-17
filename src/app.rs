use crate::{
    components::{CanShoot, Controllable, DummyDraw, Movable, Player, Position},
    controls::Controls,
    drawable::Drawable,
    system::update_system,
    ui::{draw_fps, StageUI},
    window::Window,
};

use hecs::World;
use macroquad::prelude::*;

pub struct App {
    window: Window,
    controls: Controls,
    world: World,
}

impl App {
    /// Initialize Game state
    pub fn new(window: Window, controls: Controls) -> App {
        let mut world = World::new();

        world.spawn((
            Player,
            Controllable,
            Movable::default(),
            Position::default(),
            CanShoot::default(),
            DummyDraw,
        ));

        Self {
            window,
            controls,
            world,
        }
    }

    /// This is where the update happen
    pub async fn update(&mut self) {}

    /// This is where the draw happen
    pub async fn draw(&mut self) {
        clear_background(BLACK);

        update_system(&mut self.world, &self.controls, &self.window);
        StageUI::draw(&self.window).await;

        draw_fps(&self.window, 32.0, WHITE);
    }
}
