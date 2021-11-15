use ::std::fmt::Write;
use std::cmp::max;

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
        Err(err) => Err(match err {
            ParseError::InvalidToken { location } => {
                let (line , col) = source_line_col(code, location);
                format!("Invalid code in {}:{}:{}\n{}",
                        identifier, line + 1, col + 1, source_loc_repr(code, line, col, 1))
            },
            ParseError::UnrecognizedEOF { location, expected } => {
                let (line, col) = source_line_col(code, location);
                format!("Unexpected end in {}:{}:{}\n{}\nExpected one of: {}",
                        identifier, line + 1, col + 1, source_loc_repr(code, line, col, 1), expected.join(" / "))
            },
            ParseError::UnrecognizedToken { token: (start, _, end), expected } => {
                let (line, col) = source_line_col(code, start);
                format!("Unexpected code in {}:{}:{}\n{}\nExpected one of: {}",
                        identifier, line + 1, col + 1, source_loc_repr(code, line, col, max(1, end - start)), expected.join(" / "))
            },
            ParseError::ExtraToken { token: (start, _, end) } => {
                let (line, col) = source_line_col(code, start);
                format!("Invalid token in {}:{}:{}\n{}",
                        identifier, line + 1, col + 1, source_loc_repr(code, line, col, max(1, end - start)))
            },
            ParseError::User { error } => {
                format!("Error in {}: {}", identifier, error)
            },
        }),
    }
}

fn source_line_col(code: &str, start: usize) -> (usize, usize) {
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
    (err_line_nr, err_char_in_line)
}

fn source_loc_repr(code: &str, err_line: usize, err_col: usize, len: usize) -> String {
    assert!(len >= 1);
    let mut locator = String::with_capacity(160);
    for (line_nr, line) in code.lines().enumerate() {
        if line_nr + 2 > err_line {
            write!(locator, "{:3} | {}\n", line_nr + 1, line);
        }
        if line_nr == err_line {
            write!(locator, "      {}{} {}{}\n", " ".repeat(err_col), "^".repeat(len),
                   err_col + 1, if len > 1 { format!("-{}", err_col + len) } else { "".to_owned() });
        }
        if line_nr > err_line + 2 {
            break
        }
    }
    locator
}
