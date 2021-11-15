use ::std::fmt::Write;

use ::lalrpop_util::ParseError;

use crate::common::ApivResult;
use crate::load::ast::EvolutionAst;

use self::grammar::grammar::evolutionParser;

pub mod grammar {
    #![allow(warnings)]

    use ::lalrpop_util::lalrpop_mod;

    lalrpop_mod!(pub grammar);
}

pub fn compile(identifier: &str, code: &str) -> ApivResult<EvolutionAst> {
    match evolutionParser::new().parse(code) {
        Ok(ast) => Ok(ast),
        Err(err) => match err {
            ParseError::InvalidToken { location } => Err(format!("Invalid token in '{}':\n{}", identifier, source_locator(code, location, 1))),
            ParseError::UnrecognizedEOF { .. } => Err(unimplemented!()),
            ParseError::UnrecognizedToken { .. } => Err(unimplemented!()),
            ParseError::ExtraToken { .. } => Err(unimplemented!()),
            ParseError::User { .. } => Err(unimplemented!()),
        },
    }
}

fn source_locator(code: &str, start: usize, len: usize) -> String {
    assert!(len >= 1);
    let mut err_line_nr = 0;
    let mut err_char_in_line = 0;
    let mut char_nr = 0;
    for line in code.lines() {
        if char_nr + line.len() >= start {
            err_char_in_line = start - char_nr;
            break
        }
        char_nr += line.len() + 1;
        err_line_nr += 1;
    }
    let mut locator = String::with_capacity(160);
    for (line_nr, line) in code.lines().enumerate() {
        if line_nr + 2 > err_line_nr {
            write!(locator, "{:3} | {}\n", line_nr + 1, line);
        }
        if line_nr == err_line_nr {
            write!(locator, "      {}{}\n", " ".repeat(err_char_in_line), "^".repeat(len));
        }
        if line_nr > err_line_nr + 2 {
            break
        }
    }
    locator
}
