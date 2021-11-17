use ::std::env;
use ::std::path::PathBuf;

use ::lalrpop;

fn main() {
    parse_grammar_definition();
}

fn parse_grammar_definition() {
    println!("cargo:rerun-if-changed=grammar.lalrpop");
    let in_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    lalrpop::Configuration::new()
        .set_in_dir(in_dir)
        .process()
        .unwrap();
}
