pub enum GameState {
    GameRunning,
    GamePaused,
    GameOver,
    MainMenu(MainMenuState),
}

pub enum MainMenuState {
    TitleScreen,
    Option,
    HighScore,
}
