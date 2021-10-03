mod util;

use rt_easy_compiler::{CompilerErrorKind, Error};

#[test]
fn bit_range_too_wide() {
    const SOURCES: &'static [&'static str] = &[
        r#"
            declare bus B(47594590:25)
        "#,
        r#"
            declare register B(65536:0)
        "#,
        r#"
            declare register array ARR(66666666:0)[32]
        "#,
    ];

    for source in SOURCES {
        match util::check_err(source) {
            Error::Errors(errors) => {
                for error in errors {
                    match error.kind {
                        CompilerErrorKind::BitRangeTooWide { .. } => (),
                        other => panic!("unexpected error: {:?}", other),
                    }
                }
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }
}

#[test]
fn duplicate_symbol() {
    const SOURCES: &'static [&'static str] = &[
        r#"
            declare register A(3:0)
            declare bus A
            declare input A
        "#,
        r#"
            declare register A(3:0), X
            declare memory A(X,X)
            declare register X(2)
        "#,
        r#"
            declare bus A(3:0)
            declare register array B[32], A(7)[4]
        "#,
    ];

    for source in SOURCES {
        match util::check_err(source) {
            Error::Errors(errors) => {
                for error in errors {
                    match error.kind {
                        CompilerErrorKind::DuplicateSymbol(..) => (),
                        other => panic!("unexpected error: {:?}", other),
                    }
                }
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }
}
