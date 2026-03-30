use serde::Serialize;
use std::fmt;
use std::str::FromStr;

use super::errors;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub enum OMathVerticalAlignmentType {
    Top,
    Center,
    Bottom,
}

impl fmt::Display for OMathVerticalAlignmentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OMathVerticalAlignmentType::Top => write!(f, "top"),
            OMathVerticalAlignmentType::Center => write!(f, "center"),
            OMathVerticalAlignmentType::Bottom => write!(f, "bottom"),
        }
    }
}

impl FromStr for OMathVerticalAlignmentType {
    type Err = errors::TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "top" => Ok(Self::Top),
            "center" => Ok(Self::Center),
            "bottom" | "bot" => Ok(Self::Bottom),
            _ => Err(errors::TypeError::Unsupported(s.to_string())),
        }
    }
}
