use std::{convert::TryFrom, fmt, str::FromStr};

use crate::error::ChError;

#[derive(PartialEq, Copy, Clone)]
pub enum Version {
    Http10,
    Http11,
}

impl Version {
    fn as_str(&self) -> &str {
        match self {
            Version::Http10 => "HTTP/1.0",
            Version::Http11 => "HTTP/1.1",
        }
    }
}

impl TryFrom<&str> for Version {
    type Error = ChError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(ChError::EmptyVersion)
        } else {
            match value.to_uppercase().as_str() {
                "HTTP/1.0" => Ok(Version::Http10),
                "HTTP/1.1" => Ok(Version::Http11),
                _ => Err(ChError::VersionUnsupported(value.to_owned())),
            }
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Version::Http11
    }
}

impl FromStr for Version {
    type Err = ChError;

    fn from_str(s: &str) -> Result<Self, ChError> {
        if s.is_empty() {
            Err(ChError::EmptyVersion)
        } else {
            match s.to_uppercase().as_str() {
                "HTTP/1.0" => Ok(Version::Http10),
                "HTTP/1.1" => Ok(Version::Http11),
                _ => Err(ChError::VersionUnsupported(s.to_owned())),
            }
        }
    }
}

impl TryFrom<Option<&str>> for Version {
    type Error = ChError;

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        Version::from_str(value.unwrap_or(""))
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let version = self.as_str();

        write!(f, "{}", version)
    }
}
