use scarlet_project::app::App;
use scarlet_project::controls::Controls;
use scarlet_project::utils::setup;
use scarlet_project::window::{window_conf, Window};

use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() -> Result<(), color_eyre::Report> {
    setup()?;

    let window = Window::default();
    let controls = Controls::default();
    let mut app = App::new(window, controls).await;

    loop {
        app.update();
        app.draw().await;
        next_frame().await;
    }

    Ok(())
}
