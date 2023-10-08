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
    GCodeParseError(String, usize),
    #[error("Unknown Instruction Type {0}")]
    UnknownInstructionType(String),
    #[error("Setup Error {0}")]
    SetupError(String),
    #[error("GCode State Parse Error {0}")]
    GCodeStateParseError(String),
    #[error("Settings Load Error {0}")]
    SettingsLoadError(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
