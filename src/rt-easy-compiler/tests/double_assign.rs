mod util;

use rt_easy_compiler::{CompilerErrorKind, Error};

#[test]
fn double_assign_full() {
    const SOURCE: &'static str = r#"
        declare bus B(7:0)
        B <- 12, B <- 0b001;
    "#;

    match util::check_err(SOURCE) {
        Error::Errors(errors) => {
            for error in errors {
                match error.kind {
                    CompilerErrorKind::DoubleAssign(..) => (),
                    other => panic!("unexpected error: {:?}", other),
                }
            }
        }
        other => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn double_assign_partial() {
    const SOURCE: &'static str = r#"
        declare bus B(7:0)
        B(3:1) <- 4, B(6:3) <- 0b001;
    "#;

    match util::check_err(SOURCE) {
        Error::Errors(errors) => {
            for error in errors {
                match error.kind {
                    CompilerErrorKind::DoubleAssign(..) => (),
                    other => panic!("unexpected error: {:?}", other),
                }
            }
        }
        other => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn double_assign_mixed() {
    const SOURCE: &'static str = r#"
        declare bus B(7:0)
        B <- 4, B(6:3) <- 7;
    "#;

    match util::check_err(SOURCE) {
        Error::Errors(errors) => {
            for error in errors {
                match error.kind {
                    CompilerErrorKind::DoubleAssign(..) => (),
                    other => panic!("unexpected error: {:?}", other),
                }
            }
        }
        other => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn double_assign_if() {
    const SOURCE: &'static str = r#"
        declare bus B(7:0)
        if 1 then B <- 4 fi, B(6:3) <- 7;
    "#;

    match util::check_err(SOURCE) {
        Error::Errors(errors) => {
            for error in errors {
                match error.kind {
                    CompilerErrorKind::DoubleAssign(..) => (),
                    other => panic!("unexpected error: {:?}", other),
                }
            }
        }
        other => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn no_double_assign() {
    const SOURCE: &'static str = r#"
        declare register X(7:0)
        if X(0) then X <- 2 else X <- 7 fi;
    "#;

    util::check(SOURCE);
}
