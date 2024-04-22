use kurai_project::{kurai_window, Game};

#[macroquad::main(kurai_window)]
async fn main() {
    Game::new().await.game_loop().await;
}
