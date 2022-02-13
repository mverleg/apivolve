use ::std::cmp::Ordering;
use ::std::fmt;
use ::std::path::Path;
use std::fmt::Formatter;

use ::lazy_static::lazy_static;
use ::regex::Regex;
use ::serde::{Serialize, Serializer};
use ::serde::{Deserialize, Deserializer};
use serde::de::{DeserializeOwned, Error, Visitor};

use crate::ApivResult;

lazy_static! {
    static ref VERSION_RE: Regex =
        Regex::new(r"^v([0-9]+)\.([0-9]+)\.([0-9]+)(\.[a-zA-Z0-9_\-]+)?$").unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl TryFrom<&str> for Version {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let groups = VERSION_RE.captures(value).ok_or_else(|| {
            format!(
                "version should be a 'v' and 3 numbers separated by periods, like v1.2.3, not '{}'",
                &value
            )
        })?;
        let desc = groups.get(4).map(|m| m.as_str().to_owned());
        if let Some(desc) = desc {
            return Err(format!(
                "version should be just 3 numbers, a suffix '{}' is not allowed",
                &desc
            ));
        }
        Ok(Version {
            major: groups[1].parse().unwrap(),
            minor: groups[2].parse().unwrap(),
            patch: groups[3].parse().unwrap(),
        })
    }
}

struct VersionDeserializeVisitor();

impl<'de> Visitor<'de> for VersionDeserializeVisitor {
    type Value = Version;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a version string, i.e. '1.2.4' or '0.3.17'")
    }

    fn visit_str<E>(self, text: &str) -> Result<Self::Value, E> where E: Error {
        Version::parse(text)
    }

    fn visit_string<E>(self, text: String) -> Result<Self::Value, E> where E: Error {
        Version::parse(&text)
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(VersionDeserializeVisitor())
    }
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Version {
            major,
            minor,
            patch,
        }
    }

    pub fn parse(text: &str) -> ApivResult<Version> {
        assert!(!text.contains("-"), "dash (-) in version not yet supported");
        let parts = text.split(".")
            .map(|part| part.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| format!("failed to parse part of version as a positive number: {}, err {}", text, err))?;
        if parts.len() < 3 {
            return Err(format!("version should have at least three numbers separated by periods; this is too short: {}", text))
        }
        if parts.len() < 3 {
            return Err(format!("version should have no more than three numbers separated by periods; this is too long: {}", text))
        }
        Ok(Version::new(parts[0], parts[1], parts[2]))
    }

    pub fn zero() -> Self {
        Version::new(0, 0, 0)
    }

    pub fn pure(&self) -> Version {
        Version {
            major: self.major,
            minor: self.minor,
            patch: self.patch,
        }
    }

    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }

    pub fn patch(&self) -> u32 {
        self.patch
    }

    /// Return an ancestor by decrementing the last non-zero item. Not necessarily the direct
    /// ancestor, i.e. v1.2.0 returns v1.1.0 but direct one could be v1.1.4. Version 0.0.0 returns itself.
    pub fn ancestor(&self) -> Version {
        if self.patch > 0 {
            return Version::new(self.major, self.minor, self.patch - 1);
        }
        if self.minor > 0 {
            return Version::new(self.major, self.minor - 1, 0);
        }
        if self.major > 0 {
            return Version::new(self.major - 1, 0, 0);
        }
        Version::new(0, 0, 0)
    }
}

impl PartialOrd<Self> for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let maj = self.major.cmp(&other.major);
        if maj != Ordering::Equal {
            return Some(maj);
        }
        let min = self.minor.cmp(&other.minor);
        if min != Ordering::Equal {
            return Some(min);
        }
        Some(self.patch.cmp(&other.patch))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deser_version() {
        let json: Vec<Version> = serde_json::from_str("[\"1.2.4\", \"0.3.17\"]").unwrap();
        assert_eq!(json, vec![Version::new(1, 2, 4), Version::new(0, 3, 17)])
    }
}
