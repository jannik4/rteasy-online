mod util;

use rt_easy_compiler_backend_vhdl::error::SynthError;

#[derive(Debug)]
struct Example {
    source: &'static str,
    is_ok: bool,
}

#[test]
fn unclocked_goto_dependency() {
    let examples = vec![
        Example {
            source: r#"
                declare bus B(7:0)

                START: nop;
                if B(0) then goto START fi;
            "#,
            is_ok: false,
        },
        Example {
            source: r#"
                declare bus B(7:0)

                START: nop;
                if "0".B(1) = "11" then goto START fi;
            "#,
            is_ok: false,
        },
        Example {
            source: r#"
                declare bus B(7:0)

                START: nop;
                nop | if B(0) then goto START fi;
            "#,
            is_ok: true, // TODO: ???
        },
    ];

    for example in examples {
        if example.is_ok {
            let _vdhl = util::compile(example.source);
        } else {
            match util::compile_err(example.source) {
                compiler::Error::Errors(_) | compiler::Error::Internal(_) => {
                    panic!("expected backend error")
                }
                compiler::Error::Backend(compiler::BackendError(err)) => {
                    let err = *err.downcast::<SynthError>().unwrap();
                    assert!(matches!(err, SynthError::UnclockedGotoDependency));
                }
            }
        }
    }
}

#[test]
fn conditional_goto_in_first_state() {
    let examples = vec![
        Example {
            source: r#"
                declare register X(7:0)

                if X(0) then goto END fi;
                nop;
                END: X <- 12;
            "#,
            is_ok: false,
        },
        Example {
            source: r#"
                declare register X(7:0)

                if X(0)."1" = 23 then goto END fi;
                nop;
                END: X <- 12;
            "#,
            is_ok: false,
        },
        Example {
            source: r#"
                declare register X(7:0)

                nop | if X(0) then goto END fi;
                nop;
                END: X <- 12;
            "#,
            is_ok: true,
        },
        Example {
            source: r#"
                declare register X(7:0)

                if 12 = 12 then goto END fi;
                nop;
                END: X <- 12;
            "#,
            is_ok: true,
        },
        Example {
            source: r#"
                declare register X(7:0)

                goto END;
                nop;
                END: X <- 12;
            "#,
            is_ok: true,
        },
    ];

    for example in examples {
        if example.is_ok {
            let _vdhl = util::compile(example.source);
        } else {
            match util::compile_err(example.source) {
                compiler::Error::Errors(_) | compiler::Error::Internal(_) => {
                    panic!("expected backend error")
                }
                compiler::Error::Backend(compiler::BackendError(err)) => {
                    let err = *err.downcast::<SynthError>().unwrap();
                    assert!(matches!(err, SynthError::ConditionalGotoInFirstState));
                }
            }
        }
    }
}
