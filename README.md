# Quint Rust

Porting [Quint's](https://quint-lang.org/) lexer/compiler (and other things) to Rust.  

There's an enormous amount of **interesting** work within Quint, which is currently implemented
in Typescript. This effort is to gain a deeper understanding of how everything works while exploring the potential advantages (if any) of moving things to Rust.  

The ultimate goal of this work is to help grow the Quint toolchain and learn alot along the way.

## Lexer/Parser decisions

Why we're not using Antlr4:
* Antlr doesn't have a supported plugin for Rust
* The `antlr-rust` crate appears outdated and unsupported
* Didn't want an additional Java dependency
   
What we're using:
* [logos](https://logos.maciej.codes/intro.html) for the lexer for its ease of use, performance, and community support.
* [lalrpop](https://lalrpop.github.io/lalrpop/) to parse into the IR expected by the evaluator.  We chose it because of it's support and use on notable other projects building programming languages. It also connects nicely with Rust which is helpful in writing the grammar.

> Note: the `evaluator` crate is copied directly from the [Quint repository](https://github.com/informalsystems/quint/tree/main/evaluator) to use it for testing and integration with the lexer/parser.

## Steps to an inital MVP
- Lexer (mostly done)
- Parser with IR transformation (at the beginning of that steep hill)
- Basic example of interacting with the interpreter - simple repl.

Example:
```rust
// create the lexer and parser and parse the expression
let parsed = parse_quint_expr("(1+(2*3))");
assert!(parsed.is_ok());

// run the interpreter
let value = run(&LookupTable::default(), &parsed.unwrap());

assert_eq!(7i64, value.unwrap().as_int());
```

For more, see the [basic integration tests](https://github.com/davebryson/quint-rs/blob/main/parser/tests/integration.rs#L44) for evaluating simple expressions.



