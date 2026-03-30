use serde::Serialize;
use std::fmt;
use std::str::FromStr;

use super::errors;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub enum OMathBreakBinaryOperatorType {
    Before,
    After,
    Repeat,
}

impl fmt::Display for OMathBreakBinaryOperatorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Before => write!(f, "before"),
            Self::After => write!(f, "after"),
            Self::Repeat => write!(f, "repeat"),
        }
    }
}

impl FromStr for OMathBreakBinaryOperatorType {
    type Err = errors::TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "before" => Ok(Self::Before),
            "after" => Ok(Self::After),
            "repeat" => Ok(Self::Repeat),
            _ => Err(errors::TypeError::Unsupported(s.to_string())),
        }
    }
}
