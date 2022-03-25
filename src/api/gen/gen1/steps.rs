use ::apivolve_generator_api::gen1::GenerateSteps;

use crate::{ApivResult, FullEvolution};

pub fn evolutions_to_steps(x: &FullEvolution) -> ApivResult<GenerateSteps> {
    //TODO @mark:
    Ok(GenerateSteps {
        versions: vec![],
        pending: vec![]
    })
}
