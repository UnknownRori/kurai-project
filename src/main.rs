use scarlet_project::app::App;
use scarlet_project::window::window_conf;

use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = App::new();

    loop {
        app.update().await;
        app.draw().await;
        next_frame().await
    }
}
