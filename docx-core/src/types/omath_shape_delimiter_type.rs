use serde::Serialize;
use std::fmt;
use std::str::FromStr;

use super::errors;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub enum OMathShapeDelimiterType {
    Centered,
    Match,
}

impl fmt::Display for OMathShapeDelimiterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OMathShapeDelimiterType::Centered => write!(f, "centered"),
            OMathShapeDelimiterType::Match => write!(f, "match"),
        }
    }
}

impl FromStr for OMathShapeDelimiterType {
    type Err = errors::TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "centered" => Ok(Self::Centered),
            "match" => Ok(Self::Match),
            _ => Err(errors::TypeError::Unsupported(s.to_string())),
        }
    }
}
