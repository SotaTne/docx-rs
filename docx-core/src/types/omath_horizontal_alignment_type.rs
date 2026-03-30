use serde::Serialize;
use std::fmt;
use std::str::FromStr;

use super::errors;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub enum OMathHorizontalAlignmentType {
    Left,
    Center,
    Right,
}

impl fmt::Display for OMathHorizontalAlignmentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OMathHorizontalAlignmentType::Left => write!(f, "left"),
            OMathHorizontalAlignmentType::Center => write!(f, "center"),
            OMathHorizontalAlignmentType::Right => write!(f, "right"),
        }
    }
}

impl FromStr for OMathHorizontalAlignmentType {
    type Err = errors::TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(Self::Left),
            "center" => Ok(Self::Center),
            "right" => Ok(Self::Right),
            _ => Err(errors::TypeError::Unsupported(s.to_string())),
        }
    }
}
