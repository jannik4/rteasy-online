mod util;

use rt_easy_compiler::{CompilerErrorKind, Error};

#[test]
fn feeback_loop_bus_to_bus() {
    const SOURCE: &'static str = r#"
        declare bus B(7:0), Q
        Q(0) <- B(0), B(0) <- Q(0);
    "#;

    match util::check_err(SOURCE) {
        Error::Errors(errors) => {
            for error in errors {
                match error.kind {
                    CompilerErrorKind::FeedbackLoop => (),
                    other => panic!("unexpected error: {:?}", other),
                }
            }
        }
        other => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn feeback_loop_if() {
    const SOURCE: &'static str = r#"
        declare bus B
        if B(0) then B <- 0 fi;
    "#;

    match util::check_err(SOURCE) {
        Error::Errors(errors) => {
            for error in errors {
                match error.kind {
                    CompilerErrorKind::FeedbackLoop => (),
                    other => panic!("unexpected error: {:?}", other),
                }
            }
        }
        other => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn no_feeback_loop_bus_to_bus() {
    const SOURCE: &'static str = r#"
        declare bus B(1:0), Q(8:7)
        Q(8) <- B(0), B(1) <- Q(7);
    "#;

    util::check(SOURCE);
}
