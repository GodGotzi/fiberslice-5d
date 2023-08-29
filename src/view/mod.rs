pub mod buffer;
pub mod camera;
pub mod environment;
pub mod visualization;

#[allow(dead_code)]
pub enum Orientation {
    Default,
    Diagonal,
    Top,
    Left,
    Right,
    Front,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Mode {
    Preview,
    Prepare,
    ForceAnalytics,
}
