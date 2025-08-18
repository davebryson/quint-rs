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
    println!("----");
    println!("{:?}", parsed);
    println!("----");
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

#[test]
fn check_list_and_indexed() {
    //let r = with_value("1.to(3)").unwrap();
    //println!("{:?}", r);

    // literal list
    let l0 = with_value("[]").unwrap();
    assert!(l0.cardinality() == 0);
    let l1 = with_value("[1]").unwrap();
    assert!(l1.cardinality() == 1);
    let l2 = with_value("[1,2,3,4]").unwrap();
    assert!(l2.cardinality() == 4);

    // index into list
    assert_eq!(1, with_value("[1][0]").unwrap().as_int());
    assert_eq!(3, with_value("[1,2,3,4][2]").unwrap().as_int());

    check_expr!("2^2", 4i64, i64);
    check_expr!("2^1", 2i64, i64);
    check_expr!("2^0", 1i64, i64);
    check_expr!("-1", -1i64, i64);

    check_expr!("10 / 5 * 2", 4i64, i64);
    //check_expr!("10 % 2", 0i64, i64);
    //check_expr!("11 % 2", 1i64, i64);
}

#[test]
fn literal_list() {
    let l0 = with_value("[]").unwrap();
    assert!(l0.cardinality() == 0);
    let l1 = with_value("[1]").unwrap();
    assert!(l1.cardinality() == 1);
    let l2 = with_value("[1,2,3,4]").unwrap();
    assert!(l2.cardinality() == 4);
    let l3 = with_value("List(1,2)").unwrap();
    let l4 = with_value("[1,2]").unwrap();
    assert!(l3.cardinality() == 2);
    assert_eq!(l3, l4);

    assert_eq!(3, with_value("[1,2,3,4][2]").unwrap().as_int());
}

#[test]
fn do_set() {
    check_expr!("10 + 2", 12i64, i64);
    check_expr!("10 - 2", 8i64, i64);
    check_expr!("10 - 2 + 1", 9i64, i64);
    check_expr!("10 * 2 + 1", 21i64, i64);
    check_expr!("10 / 2", 5i64, i64);
    check_expr!("2^2", 4i64, i64);
    check_expr!("(1+1)^2", 4i64, i64);

    assert!(with_value("Set()").is_ok());
    assert!(with_value("Set(1,2,3)").is_ok());
    assert!(with_value("Set(1,2,3).union(Set(4))").is_ok());
    assert!(with_value("[1,2,3]").is_ok());
    // NOTE below works, but should also handle: [1,2,3].tail()
    // See NormalCallName
    assert!(with_value("List(1,2,3).tail()").is_ok());
    check_expr!("List(1,2,3).nth(1)", 2i64, i64);
}

#[test]
fn lists() {
    assert!(with_value("[]").is_ok());
    assert!(with_value("[1,2,3]").is_ok());
    assert!(with_value("List(4,5,6)").is_ok());
    assert!(with_value("List(4,5,6).head()").is_ok());
    assert!(with_value("[4,5,6].head()").is_ok());
    check_expr!("[1,2,3][1]", 2i64, i64);
    check_expr!("List(2,3,4)[2]", 4i64, i64);

    assert!(with_value("Set(1,2,3).union(Set(4))").is_ok());
    assert!(with_value("Set(Set(1))").is_ok());

    check_expr!("true or false", true, bool);
    check_expr!("true and false", false, bool);
    check_expr!("true iff true", true, bool);

    check_expr!("all { true, false }", false, bool);
    check_expr!("any { true, false }", true, bool);

    check_expr!("and { true, true }", true, bool);
}
