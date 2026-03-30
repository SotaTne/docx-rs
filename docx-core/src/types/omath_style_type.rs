use serde::Serialize;
use std::fmt;
use std::str::FromStr;

use super::errors;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub enum OMathStyleType {
    Plain,
    Bold,
    Italic,
    BoldItalic,
}

impl fmt::Display for OMathStyleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Plain => write!(f, "p"),
            Self::Bold => write!(f, "b"),
            Self::Italic => write!(f, "i"),
            Self::BoldItalic => write!(f, "bi"),
        }
    }
}

impl FromStr for OMathStyleType {
    type Err = errors::TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "p" => Ok(Self::Plain),
            "b" => Ok(Self::Bold),
            "i" => Ok(Self::Italic),
            "bi" => Ok(Self::BoldItalic),
            _ => Err(errors::TypeError::Unsupported(s.to_string())),
        }
    }
}
