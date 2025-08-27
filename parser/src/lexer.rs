//!
//! Lexer for Quint
//!
use std::num::ParseIntError;

use logos::{Lexer, Logos, SpannedIter};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger(ParseIntError),
    #[default]
    InvalidToken,
}

impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        LexicalError::InvalidInteger(err)
    }
}

pub struct QuintLexer<'input> {
    token_stream: SpannedIter<'input, Token>,
}

impl<'input> QuintLexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            token_stream: Token::lexer(input).spanned(),
        }
    }
}

impl Iterator for QuintLexer<'_> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream
            .next()
            .map(|(token, span)| Ok((span.start, token?, span.end)))
    }
}

// remove qoutes from tokenized stings
fn remove_quotes(lex: &mut Lexer<Token>) -> String {
    lex.slice().trim_matches(move |c| c == '\"').to_string()
}

fn num_callback(lex: &mut Lexer<Token>) -> Option<i64> {
    lex.slice().replace("_", "").parse().ok()
}

fn hex_callback(lex: &mut Lexer<Token>) -> Option<i64> {
    let without_underscores = lex.slice().replace("_", "");
    let without_prefix = without_underscores.trim_start_matches("0x");
    i64::from_str_radix(without_prefix, 16).ok()
}

/// Lexer for the Quint Language
///
/// Note: issues with multi-line comments:
/// This r"/\*([^*]|\*+[^*/])*\*+/" works but has backtrack issues. See notes
/// on Comment (below)
///
#[derive(Debug, Logos, PartialEq, Eq, Hash, Clone)]
// skip: tabs,newlines,returns,double/triple line comments.
// Note we handle block comments below.
#[logos(
    skip r"[ \t\r\n]+", 
    skip r"//[^\n]*", 
    skip r"///[^\n]*", 
    error = LexicalError)
]
pub enum Token {
    // Literals
    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Bool(bool),

    // extract a quoted string and remove quotes
    #[regex(r#"(?:"([^"]*)")"#, remove_quotes)]
    String(String),

    // parse number/hex
    #[regex(r"\d+[_\d]*", num_callback, priority = 1)]
    Int(i64),
    #[regex(r"0x[0-9a-fA-F_]+", hex_callback, priority = 1)]
    Hex(i64),

    // Note: remove Int AFTER the '_'  This caused confusion with the tuple
    // operatoe '()._1' as the _1 gets picked up as an identifier vs an int
    #[regex(r"([a-z][a-zA-Z0-9_]*|[_][a-zA-Z_]+)", |lex| lex.slice().parse::<String>().ok(), priority=0)]
    LowId(String),
    #[regex(r"([A-Z][a-zA-Z0-9_]*|[_][a-zA-Z_]+)", |lex| lex.slice().parse::<String>().ok())]
    CapId(String),

    // Operators
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("%")]
    Mod,

    #[token("=")]
    Assign,
    #[token("==")]
    EQ,
    #[token("!=")]
    NE,
    #[token(">")]
    GT,
    #[token("<")]
    LT,
    #[token(">=")]
    GE,
    #[token("<=")]
    LE,

    // Structure
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(";")]
    SemiColon,
    #[token("::")]
    DoubleColon,
    #[token(".")]
    Period,
    #[token("'")]
    Prime,
    #[token("|")]
    Pipe,
    #[token("->")]
    Arrow,
    #[token("=>")]
    DoubleArrow,
    #[token("^")]
    Hat,
    #[token("_")]
    Underscore,
    #[token("...")]
    Ellipsis,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,

    #[token("module")]
    Module,
    #[token("const")]
    Const,
    #[token("assume")]
    Assume,
    #[token("var")]
    Var,
    #[token("val")]
    Val,
    #[token("def")]
    Def,
    #[token("pure")]
    Pure,
    #[token("action")]
    Action,
    #[token("temporal")]
    Temporal,
    #[token("nondet")]
    Nondet,
    #[token("run")]
    Run,

    #[token("all")]
    All,
    #[token("any")]
    Any,
    #[token("if")]
    If,
    #[token("iff")]
    Iff,
    #[token("else")]
    Else,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("implies")]
    Implies,
    #[token("match")]
    Match,

    #[token("Set")]
    Set,
    #[token("List")]
    List,

    #[token("import")]
    Import,
    #[token("export")]
    Export,

    #[token("from")]
    From,
    #[token("as")]
    As,

    // TypeAnnotations
    #[token("type")]
    Type,
    #[token("int")]
    TypeInt,
    #[token("bool")]
    TypeBool,
    #[token("str")]
    TypeStr,

    // Work around for multi-line block comments
    // known issue: https://github.com/maciejhirsz/logos/issues/180#issuecomment-736401091
    // this means the multi-line comment is still captured with the tokens.
    #[token("/*", |lex| {
        let len = lex.remainder().find("*/")?;
        lex.bump(len + 2); // include len of `*/`
        Some(())
    })]
    Comment,
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    macro_rules! check_lexing {
        ($expr:expr, $expected:expr) => {{
            let lexer = QuintLexer::new($expr);
            let r = lexer
                .into_iter()
                .filter(|t| t.is_ok())
                .map(|v| v.unwrap().1)
                .collect::<Vec<_>>();
            assert_eq!($expected, r);
        }};
    }

    #[test]
    fn no_errors_on_module() {
        let tictac = include_str!("../fixtures/tictactoe.qnt");
        let lexer = QuintLexer::new(tictac);
        assert!(lexer.into_iter().all(|result| result.is_ok()));
    }

    #[test]
    fn error_but_continues() {
        let lexer = QuintLexer::new("pure @ x: int = 10");
        assert!(
            lexer
                .into_iter()
                .filter(|t| t.is_err())
                .collect::<Vec<_>>()
                .len()
                > 0
        );
    }

    #[test]
    fn token_structure() {
        check_lexing!("1 + 1", vec![Int(1), Add, Int(1)]);
        check_lexing!(
            "module dave {}",
            vec![Module, LowId("dave".into()), LBrace, RBrace]
        );
        check_lexing!(
            r#"module dave {
               var state: int;
               action: init: {
                  state' = state + 0
               } 
            }"#,
            vec![
                Module,
                LowId("dave".into()),
                LBrace,
                Var,
                LowId("state".into()),
                Colon,
                TypeInt,
                SemiColon,
                Action,
                Colon,
                LowId("init".into()),
                Colon,
                LBrace,
                LowId("state".into()),
                Prime,
                Assign,
                LowId("state".into()),
                Add,
                Int(0i64),
                RBrace,
                RBrace,
            ]
        );
    }
}
