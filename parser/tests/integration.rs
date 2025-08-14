use quint_evaluator::{evaluator::run, ir::LookupTable};
use quint_parser::parse_quint_expr;

#[test]
fn basic_integration() {
    // parse the expression
    let parsed = parse_quint_expr("(1+(2*3))");
    assert!(parsed.is_ok());

    // run the interpreter
    let table = LookupTable::default();
    let value = run(&table, &parsed.unwrap());

    assert_eq!(7i64, value.unwrap().as_int());
}
