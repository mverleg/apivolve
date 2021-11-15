use crate::common::ApivResult;
use crate::load::ast::EvolutionAst;

use self::grammar::grammar::evolutionParser;

pub mod grammar {
    #![allow(warnings)]

    use ::lalrpop_util::lalrpop_mod;

    lalrpop_mod!(pub grammar);
}

pub fn compile(identifier: &str, code: &str) -> ApivResult<EvolutionAst> {
    return evolutionParser::new()
        .parse(code)
        //TODO @mark: msg
        .map_err(|err| format!("{} failed to compile: {}", identifier, err))
}
