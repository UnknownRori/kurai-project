use kurai_project::{game_window, App};
use macroquad::window::next_frame;

#[macroquad::main(game_window)]
async fn main() {
    let mut app = App::new().await;
    app.game_loop().await;
}
