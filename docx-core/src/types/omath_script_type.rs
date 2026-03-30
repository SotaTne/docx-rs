use serde::Serialize;
use std::fmt;
use std::str::FromStr;

use super::errors;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub enum OMathScriptType {
    Roman,
    Script,
    Fraktur,
    DoubleStruck,
    SansSerif,
    Monospace,
}

impl fmt::Display for OMathScriptType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Roman => write!(f, "roman"),
            Self::Script => write!(f, "script"),
            Self::Fraktur => write!(f, "fraktur"),
            Self::DoubleStruck => write!(f, "double-struck"),
            Self::SansSerif => write!(f, "sans-serif"),
            Self::Monospace => write!(f, "monospace"),
        }
    }
}

impl FromStr for OMathScriptType {
    type Err = errors::TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "roman" => Ok(Self::Roman),
            "script" => Ok(Self::Script),
            "fraktur" => Ok(Self::Fraktur),
            "double-struck" => Ok(Self::DoubleStruck),
            "sans-serif" => Ok(Self::SansSerif),
            "monospace" => Ok(Self::Monospace),
            _ => Err(errors::TypeError::Unsupported(s.to_string())),
        }
    }
}
