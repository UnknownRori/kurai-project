pub struct ScoreData {
    pub graze: i32,
    pub score: i32,
    pub power: f32,
    pub value: i32,
    pub life: i8,
    pub spell: i8,
}

impl Default for ScoreData {
    fn default() -> Self {
        Self {
            graze: 0,
            score: 0,
            power: 1.0,
            value: 10000,
            life: 3,
            spell: 3,
        }
    }
}
