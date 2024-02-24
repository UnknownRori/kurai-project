pub enum Difficulty {
    Easy,    // Plebs
    Normal,  // Good
    Hard,    // Master
    Lunatic, // Master of Master
}

impl Difficulty {
    #[inline(always)]
    pub fn difficulty_value<T>(&self, easy: T, normal: T, hard: T, lunatic: T) -> T {
        match self {
            Difficulty::Easy => easy,
            Difficulty::Normal => normal,
            Difficulty::Hard => hard,
            Difficulty::Lunatic => lunatic,
        }
    }
}
