use ::std::collections::HashSet;

use crate::{ApivResult, Version};
use crate::load::read::Evolution;

pub fn linearize(mut evolutions: Vec<Evolution>) -> ApivResult<Vec<Evolution>> {
    let mut seen = HashSet::new();
    seen.insert(Version::zero());
    evolutions.sort_by(|left, right| left.version.cmp(&right.version));
    for evolution in &evolutions {
        let version = evolution.version.clone();
        if !seen.contains(&version.ancestor()) {
            return Err(format!("Found a version {} but not the expected predecessor {}", &version, version.ancestor()));
        }


        seen.insert(version);
    }
    Ok(evolutions)
}
