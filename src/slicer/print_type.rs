use strum_macros::EnumString;

#[derive(Debug, Clone, EnumString)]
pub enum PrintType {
    InternalInFill,
    SolidInFill,
    BridgeInFill,
    TopSolidInFill,
    Skirt,
    Brim,
    Support,
    Perimeter,
    ExternalPerimeter,
    OverhangPerimeter,
    Unknown,
}
