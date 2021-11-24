use ::std::collections::HashSet;

use crate::{ApivResult, Version};
use crate::load::read::{Evolution, Evolutions};

pub fn linearize(mut evolutions: Evolutions) -> ApivResult<Vec<Evolution>> {
    let mut seen = HashSet::new();
    seen.insert(Version::zero());
    evolutions.sort_by(|left, right| left.version.cmp(&right.version));
    for evolution in &evolutions {
        let version = evolution.version.clone();
        if !seen.contains(&version.ancestor()) {
            return Err(format!("Found a version {} but not the expected predecessor {}", &version, version.ancestor()));
        }
        for deps in &evolution.depends {
            //deps
        }

        seen.insert(version);
    }
    Ok(evolutions)
}
