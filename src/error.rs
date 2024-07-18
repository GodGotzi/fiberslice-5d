#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[allow(dead_code)]
    #[error("Generic {0}")]
    Generic(String),
    #[error("FieldMissing {0}")]
    FieldMissing(String),
    #[error("InitialBuild {0}")]
    InitialBuild(String),
    #[error("GCode Parse Error {0}")]
    GCodeParse(String, usize),
    #[error("Unknown Instruction Type {0}")]
    UnknownInstructionType(String),
    #[error("Setup Error {0}")]
    Setup(String),
    #[error("GCode State Parse Error {0}")]
    GCodeStateParse(String),
    #[error("Settings Load Error {0}")]
    SettingsLoad(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Ui Not Rendered")]
    UiNotRendered,
}
