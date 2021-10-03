mod util;

use rt_easy_compiler::{CompilerErrorKind, Error};

#[test]
fn goto_before_pipe() {
    const SOURCES: &'static [&'static str] = &[
        r#"
            declare register A(3:0)
            START: goto START, nop | if A(3) then nop fi;
        "#,
        r#"
            declare register A
            START: if 0 then goto START fi, nop | goto START;
        "#,
    ];

    for source in SOURCES {
        match util::check_err(source) {
            Error::Errors(errors) => {
                for error in errors {
                    match error.kind {
                        CompilerErrorKind::GotoBeforePipe => (),
                        other => panic!("unexpected error: {:?}", other),
                    }
                }
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }
}

#[test]
fn mutate_after_pipe() {
    const SOURCES: &'static [&'static str] = &[
        r#"
            declare register A(3:0), C
            START: A <- 2, nop | if A(3) then C <- 1 fi;
        "#,
        r#"
            declare register A, B
            nop | A.B <- B;
        "#,
    ];

    for source in SOURCES {
        match util::check_err(source) {
            Error::Errors(errors) => {
                for error in errors {
                    match error.kind {
                        CompilerErrorKind::MutateAfterPipe => (),
                        other => panic!("unexpected error: {:?}", other),
                    }
                }
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }
}
