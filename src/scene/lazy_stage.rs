use crate::assets::AssetsManager;

use super::{stage::Stage, stage_info::StageInfo};

pub trait LazyStage: StageInfo {
    fn build(&self, _: &AssetsManager) -> Box<dyn Stage>;
}
