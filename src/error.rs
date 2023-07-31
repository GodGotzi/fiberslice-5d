#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),
    #[error("FieldMissing {0}")]
    FieldMissing(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
