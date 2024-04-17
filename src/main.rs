use kurai_project::{game_window, App};

#[macroquad::main(game_window)]
async fn main() {
    let mut app = App::new().await;
    app.game_loop().await;
}
