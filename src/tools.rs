use macros::NumEnum;
use strum_macros::{EnumCount, EnumIter};

#[derive(Debug, Clone, Copy, NumEnum, EnumCount, EnumIter)]
pub enum Tool {
    List,
    Measure,
    Intersection,
}
