pub enum StageType {
    Story,
    Extra,
    Spell,
    Special,
}

pub trait StageInfo {
    fn id(&self) -> usize;
    fn title(&self) -> &str;
    fn sub_title(&self) -> &str;
    fn stage_type(&self) -> &StageType;
}
