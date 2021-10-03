#[test]
fn pretty_print_error() {
    const SOURCE: &'static str = r#"declare register X["#;

    const EXPECTED_ERROR: &'static str = r#" --> 1:19
  |
1 |    declare register X[
  |                      ^
  |
  = found: "[", expected one of:
           "," ...
           "(" ...
           <NUM_BIN> ...
           <NUM_HEX> ...
           <NUM_DEC> ...
           <NUM_BIT_STRING> ...
           "declare" ...
           "nop" ...
           "goto" ...
           "read" ...
           "write" ...
           "if" ...
           "switch" ...
           "assert" ...
           <ID> ..."#;

    let error = match rt_easy_parser::parse(SOURCE) {
        Ok(_) => panic!("expected error"),
        Err(e) => rt_easy_parser::pretty_print_error(&e, SOURCE, false),
    };
    assert_eq!(error, EXPECTED_ERROR);
}
