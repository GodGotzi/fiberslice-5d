use strum_macros::{EnumCount, EnumIter};

#[derive(Debug, Clone, Copy, EnumCount, EnumIter)]
pub enum Tool {
    List,
    Measure,
    Intersection,
}
