use macroquad::window::next_frame;
use kurai_project::{game_window, App};

#[macroquad::main(game_window)]
async fn main() {
    let mut app = App::new().await;
    loop {
        app.update().await;
        app.draw().await;
        next_frame().await;
    }
}
