mod util;

#[test]
fn unknown_operation() {
    const SOURCE_UNIT_TEST: &'static str = r#"
loop
    "#;

    let _error = util::compile_unit_test_err(SOURCE_UNIT_TEST);
}

#[test]
fn missing_input() {
    const SOURCE: &'static str = r#"
        declare register A(7:0), FACTOR(7:0), RES(7:0)
        declare bus INPUT(7:0)
    "#;

    const SOURCE_UNIT_TEST: &'static str = r#"
        INPUT <- 4
    "#;

    let program = util::compile(SOURCE);
    let unit_test = util::compile_unit_test(SOURCE_UNIT_TEST);

    assert!(rt_easy_unit_test::run(program, unit_test).is_err());
}
