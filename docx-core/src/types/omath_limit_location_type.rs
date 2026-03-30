use serde::Serialize;
use std::fmt;
use std::str::FromStr;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::errors;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub enum OMathLimitLocationType {
    UnderOver,
    SubscriptSuperscript,
}

impl fmt::Display for OMathLimitLocationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OMathLimitLocationType::UnderOver => write!(f, "undOvr"),
            OMathLimitLocationType::SubscriptSuperscript => write!(f, "subSup"),
        }
    }
}

impl FromStr for OMathLimitLocationType {
    type Err = errors::TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "undOvr" => Ok(OMathLimitLocationType::UnderOver),
            "subSup" => Ok(OMathLimitLocationType::SubscriptSuperscript),
            _ => Err(errors::TypeError::Unsupported(s.to_string())),
        }
    }
}
