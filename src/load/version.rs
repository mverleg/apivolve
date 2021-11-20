use ::std::cmp::Ordering;
use ::std::fmt;
use ::std::path::Path;

use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::ApivResult;

lazy_static! {
    static ref VER_RE: Regex =
        Regex::new(r"v([0-9]+)\.([0-9]+)\.([0-9]+)(\.[a-zA-Z0-9_\-]+)?\.apiv").unwrap();
}

#[derive(Debug, Clone)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    desc: Option<String>,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Version { major, minor, patch, desc: None }
    }

    pub fn pure(&self) -> Version {
        Version {
            major: self.major,
            minor: self.minor,
            patch: self.patch,
            desc: None,
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

    /// Version 0.0.0 returns itself.
    pub fn prev(&self) -> Version {
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
        match &self.desc {
            Some(desc) => write!(f, "{}.{}.{}.{}", self.major, self.minor, self.patch, desc),
            None => write!(f, "{}.{}.{}", self.major, self.minor, self.patch),
        }
    }
}

pub fn extract_version(path: &Path) -> ApivResult<Version> {
    let name_os = path.file_name().ok_or_else(|| {
        format!(
            "Could not get basename from evolution path '{}'",
            path.to_string_lossy()
        )
    })?;
    let name = name_os.to_str().ok_or_else(|| {
        format!(
            "Filename '{}' does not seem to be UTF8-encoded",
            path.to_string_lossy()
        )
    })?;
    let groups = VER_RE.captures(name).ok_or_else(|| {
        format!(
            "Evolution filename '{}' should follow a strict naming convention - \
        'v1.2.3.apiv' or 'v1.2.3.description.apiv', starting with 'v', three-digit semver, \
        optional description and ending with extension '.apiv'",
            name
        )
    })?;
    let desc = groups.get(4).map(|m| m.as_str().to_owned());
    if let Some(desc) = desc {
        //TODO: should descriptions be allowed? it is very helpful, but increases the chance to have duplicate versions without conflicts
        return Err(format!(
            "Filename should be just a version of 3 numbers, not '{}' in '{}'",
            &desc, path.to_string_lossy()
        ));
    }
    Ok(Version {
        major: groups[1].parse().unwrap(),
        minor: groups[2].parse().unwrap(),
        patch: groups[3].parse().unwrap(),
        desc: desc,
    })
}
