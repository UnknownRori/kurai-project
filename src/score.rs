pub struct ScoreData {
    pub graze: u32,
    pub score: u32,
    pub power: f32,
    pub value: u32,
    pub life: u8,
    pub spell: u8,
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
