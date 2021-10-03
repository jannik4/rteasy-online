mod util;

use rt_easy_compiler::{CompilerErrorKind, Error};

#[test]
fn condition_too_wide() {
    const SOURCES: &'static [&'static str] = &[
        r#"
            declare register A(3:0)
            if A then nop fi;
        "#,
        r#"
            declare register A
            if A.A then nop fi;
        "#,
        r#"
            if 2 then nop fi;
        "#,
        r#"
            if "01" then nop fi;
        "#,
        r#"
            declare bus B(2:0)
            assert B;
        "#,
    ];

    for source in SOURCES {
        match util::check_err(source) {
            Error::Errors(errors) => {
                for error in errors {
                    match error.kind {
                        CompilerErrorKind::ConditionTooWide(..) => (),
                        other => panic!("unexpected error: {:?}", other),
                    }
                }
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }
}
