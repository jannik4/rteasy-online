mod util;

use rt_easy_compiler::{CompilerErrorKind, Error};

#[test]
fn bit_range_too_wide() {
    const SOURCES: &'static [(&'static str, usize, usize)] = &[
        (
            r#"
            declare register A(131071:0)
            "#,
            u16::MAX as usize,
            131072,
        ),
        (
            r#"
            declare bus B(0:65536)
            "#,
            u16::MAX as usize,
            65537,
        ),
        (
            r#"
            declare register A(128:64), B
            declare memory MEM(A, B)
            "#,
            64,
            65,
        ),
    ];

    for (source, e_max_size, e_size) in SOURCES {
        match util::check_err(source) {
            Error::Errors(errors) => {
                for error in errors {
                    match error.kind {
                        CompilerErrorKind::BitRangeTooWide { max_size, size } => {
                            assert_eq!(max_size, *e_max_size);
                            assert_eq!(size, *e_size);
                        }
                        other => panic!("unexpected error: {:?}", other),
                    }
                }
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }
}
