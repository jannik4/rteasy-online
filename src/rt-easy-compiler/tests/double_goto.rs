mod util;

use rt_easy_compiler::{CompilerErrorKind, Error};

#[test]
fn double_goto_same_label() {
    const SOURCE: &'static str = r#"
        declare register X
        LABEL: goto LABEL, goto LABEL;
    "#;

    match util::check_err(SOURCE) {
        Error::Errors(errors) => {
            for error in errors {
                match error.kind {
                    CompilerErrorKind::DoubleGoto => (),
                    other => panic!("unexpected error: {:?}", other),
                }
            }
        }
        other => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn double_goto_different_label() {
    const SOURCE: &'static str = r#"
        declare register X
        LABEL: goto LABEL, goto LABEL2;
        LABEL2:
    "#;

    match util::check_err(SOURCE) {
        Error::Errors(errors) => {
            for error in errors {
                match error.kind {
                    CompilerErrorKind::DoubleGoto => (),
                    other => panic!("unexpected error: {:?}", other),
                }
            }
        }
        other => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn double_goto_if() {
    const SOURCE: &'static str = r#"
        declare register X
        LABEL: if X then goto LABEL fi, goto LABEL2;
        LABEL2:
    "#;

    match util::check_err(SOURCE) {
        Error::Errors(errors) => {
            for error in errors {
                match error.kind {
                    CompilerErrorKind::DoubleGoto => (),
                    other => panic!("unexpected error: {:?}", other),
                }
            }
        }
        other => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn no_double_goto() {
    const SOURCE: &'static str = r#"
        declare register X
        LABEL: if X then goto LABEL else goto LABEL2 fi;
        LABEL2:
    "#;

    util::check(SOURCE);
}
