use crate::load::read::Evolution;

pub fn linearize(mut evolutions: Vec<Evolution>) -> Vec<Evolution> {
    evolutions.sort_by(|left, right| left.version.cmp(&right.version));
    evolutions
}
