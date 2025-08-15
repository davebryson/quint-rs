use quint_evaluator::{evaluator::run, ir::LookupTable, value::Value};
use quint_parser::{QuintError, parse_quint_expr};

fn print_result(expr: &str) {
    let parsed = parse_quint_expr(expr);
    assert!(parsed.is_ok());
    let table = LookupTable::default();
    let val = run(&table, &parsed.unwrap());
    assert!(val.is_ok());
    println!("{:?}", val.unwrap())
}

fn with_value(expr: &str) -> Result<Value, QuintError> {
    let parsed = parse_quint_expr(expr);
    assert!(parsed.is_ok());
    let table = LookupTable::default();
    let val = run(&table, &parsed.unwrap());
    assert!(val.is_ok());
    val
}

fn eval_expr(expr: &str) -> Result<Value, QuintError> {
    let parsed = parse_quint_expr(expr);
    assert!(parsed.is_ok());
    let table = LookupTable::default();
    run(&table, &parsed.unwrap())
}

macro_rules! check_expr {
    ($expr:expr, $expected:expr, i64) => {{
        let value = eval_expr($expr);
        assert_eq!($expected, value.unwrap().as_int());
    }};
    ($expr:expr, $expected:expr, bool) => {{
        let value = eval_expr($expr);
        assert_eq!($expected, value.unwrap().as_bool());
    }};
    ($expr:expr, $expected:expr, &str) => {{
        let value = eval_expr($expr);
        assert_eq!($expected, value.unwrap().as_str());
    }};
}

#[test]
fn basic_integration() {
    check_expr!("2^2 + 1", 5i64, i64);
    check_expr!("(1+(2*3))", 7i64, i64);
    check_expr!("10 / 5 * 2", 4i64, i64);
    check_expr!("10 % 2", 0i64, i64);
    check_expr!("11 % 2", 1i64, i64);

    check_expr!("true", true, bool);
    check_expr!("false", false, bool);
    check_expr!("5 > 2", true, bool);
    check_expr!("2 < 5", true, bool);
    check_expr!("2 <= 2", true, bool);
    check_expr!("2 == 2", true, bool);
    check_expr!("2 != 3", true, bool);
    check_expr!("2 > 3", false, bool);

    check_expr!("true and true", true, bool);
    check_expr!("true and false", false, bool);
    check_expr!("false and false", false, bool);

    check_expr!("true or false", true, bool);
    check_expr!("true or true", true, bool);
    check_expr!("false or false", false, bool);

    check_expr!("true implies true", true, bool);
    check_expr!("true implies false", false, bool);

    // if then else
    check_expr!("if (1==1) true else false", true, bool);
    check_expr!("if (false) 0 else 10", 10i64, i64);

    check_expr!("\"hello\"", "hello", &str);
}

#[test]
fn check_list() {
    let r = with_value("[1,2,3]").unwrap();
    assert!(r.cardinality() == 3);
}
