use serde::Serialize;
use std::fmt;
use std::str::FromStr;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::errors;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub enum OMathFractionType {
    Bar,
    Skewed,
    Linear,
    NoBar,
}

impl fmt::Display for OMathFractionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OMathFractionType::Bar => write!(f, "bar"),
            OMathFractionType::Skewed => write!(f, "skw"),
            OMathFractionType::Linear => write!(f, "lin"),
            OMathFractionType::NoBar => write!(f, "noBar"),
        }
    }
}

impl FromStr for OMathFractionType {
    type Err = errors::TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bar" => Ok(OMathFractionType::Bar),
            "skw" => Ok(OMathFractionType::Skewed),
            "lin" => Ok(OMathFractionType::Linear),
            "noBar" => Ok(OMathFractionType::NoBar),
            _ => Err(errors::TypeError::Unsupported(s.to_string())),
        }
    }
}
