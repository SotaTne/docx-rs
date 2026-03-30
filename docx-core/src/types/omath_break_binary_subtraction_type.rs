use serde::Serialize;
use std::fmt;
use std::str::FromStr;

use super::errors;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub enum OMathBreakBinarySubtractionType {
    MinusMinus,
    MinusPlus,
    PlusMinus,
}

impl fmt::Display for OMathBreakBinarySubtractionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MinusMinus => write!(f, "--"),
            Self::MinusPlus => write!(f, "-+"),
            Self::PlusMinus => write!(f, "+-"),
        }
    }
}

impl FromStr for OMathBreakBinarySubtractionType {
    type Err = errors::TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "--" => Ok(Self::MinusMinus),
            "-+" => Ok(Self::MinusPlus),
            "+-" => Ok(Self::PlusMinus),
            _ => Err(errors::TypeError::Unsupported(s.to_string())),
        }
    }
}
