use serde::Serialize;
use std::fmt;
use std::str::FromStr;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::errors;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub enum OMathJustificationType {
    Left,
    Right,
    Center,
    CenterGroup,
}

impl fmt::Display for OMathJustificationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OMathJustificationType::Left => write!(f, "left"),
            OMathJustificationType::Right => write!(f, "right"),
            OMathJustificationType::Center => write!(f, "center"),
            OMathJustificationType::CenterGroup => write!(f, "centerGroup"),
        }
    }
}

impl FromStr for OMathJustificationType {
    type Err = errors::TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(OMathJustificationType::Left),
            "right" => Ok(OMathJustificationType::Right),
            "center" => Ok(OMathJustificationType::Center),
            "centerGroup" => Ok(OMathJustificationType::CenterGroup),
            _ => Err(errors::TypeError::Unsupported(s.to_string())),
        }
    }
}
