use crate::common::ApivResult;
use crate::load::compile::grammar::grammar::bodyParser;


pub mod grammar {
    #![allow(warnings)]

    use ::lalrpop_util::lalrpop_mod;

    lalrpop_mod!(pub grammar);
}

pub fn compile(code: &str) -> ApivResult<()> {
    return bodyParser::new()
        .parse(code)
        //TODO @mark: msg
        .map_err(|err| format!("failed to compile: {}", err))
}
