use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum OMathBarPositionType {
    Top,
    Bottom,
}

impl fmt::Display for OMathBarPositionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OMathBarPositionType::Top => write!(f, "top"),
            OMathBarPositionType::Bottom => write!(f, "bot"),
        }
    }
}

impl FromStr for OMathBarPositionType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "top" => Ok(OMathBarPositionType::Top),
            "bot" => Ok(OMathBarPositionType::Bottom),
            _ => Err(()),
        }
    }
}
