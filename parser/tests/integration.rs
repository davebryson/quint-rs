// Temporary testing.
// This should use the evaluator tests once full grammar is supported
//
use quint_evaluator::{evaluator::run, ir::LookupTable, value::Value};
use quint_parser::{QuintError, parse_quint_expr};

fn with_value(expr: &str) -> Result<Value, QuintError> {
    let parsed = parse_quint_expr(expr);
    //println!("----");
    //println!("{:?}", parsed);
    //println!("----");
    assert!(parsed.is_ok());
    let table = LookupTable::default();
    let val = run(&table, &parsed.unwrap());
    assert!(val.is_ok());
    val
}

fn eval_expr(expr: &str) -> Result<Value, QuintError> {
    let parsed = parse_quint_expr(expr);
    //assert!(parsed.is_ok());
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
    check_expr!("(1+1)^2", 4i64, i64);
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

    check_expr!("if (1==1) true else false", true, bool);
    check_expr!("if (false) 0 else 10", 10i64, i64);

    check_expr!("all { true, false }", false, bool);
    check_expr!("any { true, false }", true, bool);
    check_expr!("and { true, true }", true, bool);

    check_expr!("\"hello\"", "hello", &str);

    assert!(with_value("[]").is_ok());
    assert!(with_value("Set()").is_ok());
    assert!(with_value("Set(1,2,3)").is_ok());
    assert!(with_value("Set(1,2,3).union(Set(4))").is_ok());
    assert!(with_value("[1,2,3]").is_ok());
    assert!(with_value("List(1,2,3).tail()").is_ok());
    check_expr!("List(1,2,3).nth(1)", 2i64, i64);
    check_expr!("[1,2,3][1]", 2i64, i64);
    assert!(with_value("1.to(10)").unwrap().cardinality() == 10);
}
