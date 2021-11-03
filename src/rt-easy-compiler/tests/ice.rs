mod util;

use rt_easy_compiler::Error;

#[test]
fn ice() {
    const SOURCE: &'static str = r#"
        goto MOON;
    "#;

    match util::check_err(SOURCE) {
        Error::Internal(_ice) => (),
        other => panic!("unexpected error: {:?}", other),
    }
}
