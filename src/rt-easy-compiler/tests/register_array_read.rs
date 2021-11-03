mod util;

use rt_easy_compiler::{CompilerErrorKind, Error};

#[test]
fn register_array_read() {
    const SOURCE: &'static str = r#"
        declare register array ARR(7:0)[64]
        declare register X(7:0)
        ARR[0] <- 12 + ARR[1] + ARR[1], X <- ARR[2];
    "#;

    match util::check_err(SOURCE) {
        Error::Errors(errors) => {
            for error in errors {
                match error.kind {
                    CompilerErrorKind::RegisterArrayTooManyReads { .. } => (),
                    other => panic!("unexpected error: {:?}", other),
                }
            }
        }
        other => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn register_array_read_if() {
    const SOURCE: &'static str = r#"
        declare register array ARR(7:0)[64]
        declare register X(7:0)
        ARR[0] <- 12 + ARR[1] + ARR[1], if X(0) then X <- ARR[2] fi;
    "#;

    match util::check_err(SOURCE) {
        Error::Errors(errors) => {
            for error in errors {
                match error.kind {
                    CompilerErrorKind::RegisterArrayTooManyReads { .. } => (),
                    other => panic!("unexpected error: {:?}", other),
                }
            }
        }
        other => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn register_array_read_ok() {
    const SOURCE: &'static str = r#"
        declare register array ARR(7:0)[64]
        declare register X(7:0)
        if X(0) then ARR[0] <- 12 + ARR[1] + ARR[1] else X <- ARR[2] fi;
    "#;

    util::check(SOURCE);
}
