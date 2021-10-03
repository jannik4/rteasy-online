mod util;

use rt_easy_compiler::{CompilerErrorKind, Error};

#[test]
fn missing_goto_label() {
    const SOURCES: &'static [&'static str] = &[
        r#"
            goto X;
        "#,
        r#"
            declare register X
            X_: goto X;
        "#,
        r#"
            Y: goto X;
        "#,
    ];

    for source in SOURCES {
        match util::check_err(source) {
            Error::Errors(errors) => {
                for error in errors {
                    match error.kind {
                        CompilerErrorKind::LabelNotFound(..) => (),
                        other => panic!("unexpected error: {:?}", other),
                    }
                }
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }
}
