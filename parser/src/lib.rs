//!
//! Quint lexer and parser
//!
//!

pub mod lexer;
pub mod utils;
pub use quint_evaluator::ir::*;

use lalrpop_util::{ParseError, lalrpop_mod};

lalrpop_mod!(pub quint);

use crate::quint::QuintExParser;
use eyre::Result;
use lexer::{LexicalError, QuintLexer, Token};
use utils::QuintIdGenerator;

pub fn parse_quint_expr(content: &str) -> Result<QuintEx, ParseError<usize, Token, LexicalError>> {
    let mut generator = QuintIdGenerator::default();
    let lexer = QuintLexer::new(content);
    let parser = QuintExParser::new();
    parser.parse(&mut generator, lexer)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn initial() {
        let ast = parse_quint_expr("(1+(2*3))");
        assert!(ast.is_ok());
        println!("{:?}", ast);
    }
}
