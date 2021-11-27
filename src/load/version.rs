use ::std::cmp::Ordering;
use ::std::fmt;
use ::std::path::Path;

use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::ApivResult;

lazy_static! {
    static ref VERSION_RE: Regex =
        Regex::new(r"^v([0-9]+)\.([0-9]+)\.([0-9]+)(\.[a-zA-Z0-9_\-]+)?$").unwrap();
}

#[derive(Debug, Clone, Hash)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl TryFrom<&str> for Version {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let groups = VERSION_RE.captures(value).ok_or_else(|| {
            format!("version should be a 'v' and 3 numbers separated by periods, like v1.2.3, not '{}'", &value) })?;
        let desc = groups.get(4).map(|m| m.as_str().to_owned());
        if let Some(desc) = desc {
            return Err(format!("version should be just 3 numbers, a suffix '{}' is not allowed", &desc));
        }
        Ok(Version {
            major: groups[1].parse().unwrap(),
            minor: groups[2].parse().unwrap(),
            patch: groups[3].parse().unwrap(),
        })
    }
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Version { major, minor, patch }
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
            return Version::new(self.major, self.minor, self.patch - 1)
        }
        if self.minor > 0 {
            return Version::new(self.major, self.minor - 1, 0)
        }
        if self.major > 0 {
            return Version::new(self.major - 1, 0, 0)
        }
        Version::new(0, 0, 0)
    }
}

impl Eq for Version {}

impl PartialEq<Self> for Version {
    fn eq(&self, other: &Self) -> bool {
        self.patch == other.patch &&
            self.minor == other.minor &&
            self.major == other.major
    }
}

impl PartialOrd<Self> for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let maj = self.major.cmp(&other.major);
        if maj != Ordering::Equal {
            return Some(maj)
        }
        let min = self.minor.cmp(&other.minor);
        if min != Ordering::Equal {
            return Some(min)
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
