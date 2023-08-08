pub mod format;
pub mod frame;
pub mod task;

pub mod debug {
    use std::fmt::Debug;

    use super::format::PrettyFormat;

    pub struct DebugWrapper<T>(T);

    impl<T: PrettyFormat> Debug for DebugWrapper<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.0.pretty_format().as_str())
        }
    }

    impl<T: PrettyFormat> From<T> for DebugWrapper<T> {
        fn from(t: T) -> Self {
            Self(t)
        }
    }
}
