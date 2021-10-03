mod util;

use rt_easy_compiler::{CompilerErrorKind, Error};

#[test]
fn switch_fixed_size() {
    const SOURCES: &'static [&'static str] = &[
        r#"
            declare register A(3:0)
            switch 12 { case 1: nop default: nop };
        "#,
        r#"
            declare register A(3:0)
            switch "1" + 2 { case 0: nop default: nop };
        "#,
    ];

    for source in SOURCES {
        match util::check_err(source) {
            Error::Errors(errors) => {
                for error in errors {
                    match error.kind {
                        CompilerErrorKind::ExpectedFixedSize => (),
                        other => panic!("unexpected error: {:?}", other),
                    }
                }
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }
}

#[test]
fn switch_case_value_too_wide() {
    const SOURCES: &'static [&'static str] = &[
        r#"
            declare register A(3:0)
            switch A { case 16: nop default: nop };
        "#,
        r#"
            declare register A(3:0)
            switch "1".A { case 0b101111: nop default: nop };
        "#,
    ];

    for source in SOURCES {
        match util::check_err(source) {
            Error::Errors(errors) => {
                for error in errors {
                    match error.kind {
                        CompilerErrorKind::CaseValueTooWide { .. } => (),
                        other => panic!("unexpected error: {:?}", other),
                    }
                }
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }
}

#[test]
fn switch_case_value_constant_expr() {
    const SOURCES: &'static [&'static str] = &[
        r#"
            declare register A(3:0)
            switch A { case 4 + A: nop default: nop };
        "#,
        r#"
            declare register A(3:0)
            declare bus B
            switch "1".A { case B: nop default: nop };
        "#,
    ];

    for source in SOURCES {
        match util::check_err(source) {
            Error::Errors(errors) => {
                for error in errors {
                    match error.kind {
                        CompilerErrorKind::ExpectedConstantExpression => (),
                        other => panic!("unexpected error: {:?}", other),
                    }
                }
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }
}
