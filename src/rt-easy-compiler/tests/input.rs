mod util;

use rt_easy_compiler::{CompilerErrorKind, Error};

#[test]
fn write_to_input() {
    const SOURCES: &'static [&'static str] = &[
        r#"
            declare input IN(3:0)
            declare bus BUS(3:0)
            IN <- 2;
        "#,
        r#"
            declare input IN(3:0)
            declare bus BUS(3:0)
            BUS.IN <- 2;
        "#,
        r#"
            declare input IN(3:0)
            declare bus BUS(3:0)
            if 1 then BUS.IN <- 2 fi;
        "#,
    ];

    for source in SOURCES {
        match util::check_err(source) {
            Error::Errors(errors) => {
                for error in errors {
                    match error.kind {
                        CompilerErrorKind::AssignmentLhsContainsInput => (),
                        other => panic!("unexpected error: {:?}", other),
                    }
                }
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }
}
