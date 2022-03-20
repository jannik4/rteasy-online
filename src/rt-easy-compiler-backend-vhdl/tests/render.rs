// Test some basic invariants of rendering. In addition to that, there exist more detailed testbenches.

mod util;

#[test]
fn register_name() {
    const SOURCE: &'static str = r#"
        declare register MY_REGISTER_X

        MY_REGISTER_X <- 1;
    "#;

    let vhdl = util::compile(SOURCE).render("module", Default::default()).unwrap();
    assert!(vhdl.contains("MY_REGISTER_X"));
}

#[test]
fn label_name() {
    const SOURCE: &'static str = r#"
        MY_LABEL: nop;
    "#;

    let vhdl = util::compile(SOURCE).render("module", Default::default()).unwrap();
    assert!(vhdl.contains("MY_LABEL"));
}

#[test]
fn module_name() {
    const SOURCE: &'static str = "";

    let vhdl = util::compile(SOURCE).render("my_vhdl_module", Default::default()).unwrap();
    assert!(vhdl.contains("my_vhdl_module"));
}
