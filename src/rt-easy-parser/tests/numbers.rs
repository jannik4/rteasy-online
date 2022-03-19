mod util;

use rtcore::ast::*;
use rtcore::value::Value;

#[test]
fn binary() {
    const SOURCE: &'static str = r#"
        X <- 0b101101, X <- 0B101, X <- %001;
    "#;
    let expected = vec![
        Number { value: Value::parse_bin("101101").unwrap(), kind: NumberKind::Binary },
        Number { value: Value::parse_bin("101").unwrap(), kind: NumberKind::Binary },
        Number { value: Value::parse_bin("1").unwrap(), kind: NumberKind::Binary },
    ];

    test_numbers_util(SOURCE, &expected);
}

#[test]
fn decimal() {
    const SOURCE: &'static str = r#"
        X <- 2334, X <- 0097082, X <- 1234567890;
    "#;
    let expected = vec![
        Number { value: Value::parse_dec("2334").unwrap(), kind: NumberKind::Decimal },
        Number { value: Value::parse_dec("0097082").unwrap(), kind: NumberKind::Decimal },
        Number { value: Value::parse_dec("1234567890").unwrap(), kind: NumberKind::Decimal },
    ];

    test_numbers_util(SOURCE, &expected);
}

#[test]
fn hexadecimal() {
    const SOURCE: &'static str = r#"
        X <- 0xFF0, X <- 0X01234567890ABCDEF, X <- $FA0;
    "#;
    let expected = vec![
        Number { value: Value::parse_hex("FF0").unwrap(), kind: NumberKind::Hexadecimal },
        Number {
            value: Value::parse_hex("01234567890ABCDEF").unwrap(),
            kind: NumberKind::Hexadecimal,
        },
        Number { value: Value::parse_hex("FA0").unwrap(), kind: NumberKind::Hexadecimal },
    ];

    test_numbers_util(SOURCE, &expected);
}

#[test]
fn bit_string() {
    const SOURCE: &'static str = r#"
        X <- "101101", X <- "101", X <- "001";
    "#;
    let expected = vec![
        Number { value: Value::parse_bin("101101").unwrap(), kind: NumberKind::BitString },
        Number { value: Value::parse_bin("101").unwrap(), kind: NumberKind::BitString },
        Number { value: Value::parse_bin("001").unwrap(), kind: NumberKind::BitString },
    ];

    test_numbers_util(SOURCE, &expected);
}

#[test]
fn invalid() {
    const SOURCES: &'static [&'static str] = &[
        r#"X <- fa;"#,
        r#"X <- "fa";"#,
        r#"X <- "32";"#,
        r#"X <- $-2;"#,
        r#"X <- %FF;"#,
        r#"X <- %%12;"#,
        r#"X <- $FA$;"#,
        r#"X <- %b01;"#,
        r#"X <- $Xff;"#,
    ];

    for source in SOURCES {
        let _error = util::parse_err(source);
    }
}

fn test_numbers_util(source: &str, expected: &[Number]) {
    let ast = util::parse(source);
    let operations = &ast.statements[0].operations.operations;

    for (idx, op) in operations.iter().enumerate() {
        let expr = match op {
            Operation::Assignment(assignment) => &assignment.rhs,
            _ => panic!("Expected assignment"),
        };

        let number = match expr {
            Expression::Atom(Atom::Number(number)) => &number.node,
            _ => panic!("Expected number"),
        };

        assert_eq!(number.value, expected[idx].value);
        assert_eq!(number.value.size(), expected[idx].value.size());
        assert_eq!(number.kind, expected[idx].kind);
    }
}
