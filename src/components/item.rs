#[derive(Debug, Copy, Clone)]
pub enum DropItemType {
    Heart,
    Power,
    Value,
    BigPower,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DropItem {
    pub heart: u8,
    pub power: u8,
    pub big_power: u8,
    pub value: u8,
}

pub struct Item;
