# Quint Rust

This an effort to port [Quint's](https://quint-lang.org/) lexer/compiler (and other things) to Rust.  

There's an enormous amount of interesting work within Quint, which is currently implemented
in Typescript. This effort is to gain a deeper understanding of how everything works while exploring the potential advantages (if any) of moving things to Rust.

## Lexer/Parser decisions

Why we're not using Antlr4:
* Antlr doesn't have a supported plugin for Rust
* The `antlr-rust` crate appears outdated and unsupported
* Didn't want an additional Java dependency
   
Currently we're using:
* [logos](https://logos.maciej.codes/intro.html) as the lexer for its ease of use, performance, and community support.
* [lalrpop](https://lalrpop.github.io/lalrpop/)  for many of the same reasons as the lexer

> Note: the `evaluator` crate is copied directly from the [Quint repository](https://github.com/informalsystems/quint/tree/main/evaluator) to use it for testing and integration with the lexer/parser.

## Status
* Lexer mostly done
* Parser with IR transformation (just at the beginning of that steep hill)
* Basic example of interacting with the interpreter

Example:
```rust

// create the lexer and parser and parse the expression
let parsed = parse_quint_expr("(1+(2*3))");
assert!(parsed.is_ok());

// run the interpreter
let value = run(&LookupTable::default(), &parsed.unwrap());

assert_eq!(7i64, value.unwrap().as_int());
```



