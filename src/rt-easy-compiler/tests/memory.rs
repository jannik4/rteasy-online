mod util;

use rt_easy_compiler::{CompilerErrorKind, Error};

#[test]
fn memory_wrong_ar_dr() {
    const SOURCES: &'static [&'static str] = &[
        r#"
            declare bus B
            declare memory MEM(B,B)
        "#,
        r#"
            declare register X
            declare bus B
            declare memory MEM(X,B)
        "#,
        r#"
            declare register X
            declare bus B
            declare memory MEM(B,X)
        "#,
    ];

    for source in SOURCES {
        match util::check_err(source) {
            Error::Errors(errors) => {
                for error in errors {
                    match error.kind {
                        CompilerErrorKind::WrongSymbolType { .. } => (),
                        other => panic!("unexpected error: {:?}", other),
                    }
                }
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }
}

#[test]
fn read_write_missing_mem() {
    const SOURCES: &'static [&'static str] = &[
        r#"
            read MEM;
        "#,
        r#"
            declare register B
            write MEM;
        "#,
        r#"
            declare register MEM
            read MEM;
        "#,
        r#"
            declare register MEM
            write MEM;
        "#,
    ];

    for source in SOURCES {
        match util::check_err(source) {
            Error::Errors(errors) => {
                for error in errors {
                    match error.kind {
                        CompilerErrorKind::SymbolNotFound(..) => (),
                        CompilerErrorKind::WrongSymbolType { .. } => (),
                        other => panic!("unexpected error: {:?}", other),
                    }
                }
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }
}
