use crate::common::ApivResult;
use crate::load::ast::{Block, Evolution};

use self::grammar::grammar::evolutionParser;

pub mod grammar {
    #![allow(warnings)]

    use ::lalrpop_util::lalrpop_mod;

    lalrpop_mod!(pub grammar);
}

pub fn compile(code: &str) -> ApivResult<Evolution> {
    return evolutionParser::new()
        .parse(code)
        //TODO @mark: msg
        .map_err(|err| format!("failed to compile: {}", err))
}
