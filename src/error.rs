#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[allow(dead_code)]
    #[error("Generic {0}")]
    Generic(String),
    #[error("FieldMissing {0}")]
    FieldMissing(String),
    #[error("InitialBuild {0}")]
    InitialBuild(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
