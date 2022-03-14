mod util;

#[test]
fn zero() {
    const SOURCE: &'static str = r#"
        declare bus B(7:0)
        declare register X
    "#;

    let vhdl = util::compile(SOURCE);
    assert_eq!(vhdl.signals().control_signals.len(), 0);
}

#[test]
fn one() {
    const SOURCE: &'static str = r#"
        declare register X(3:0)

        X <- 2;
    "#;

    let vhdl = util::compile(SOURCE);
    assert_eq!(vhdl.signals().control_signals.len(), 1);
}

#[test]
fn duplicate() {
    const SOURCE: &'static str = r#"
        declare bus B(7:0)
        declare register X(3:0)

        X(3) <- 1, X(2:0) <- 2; X(2:0) <- 2;
    "#;

    let vhdl = util::compile(SOURCE);
    assert_eq!(vhdl.signals().control_signals.len(), 2);
}

#[test]
fn duplicate_full_range() {
    const SOURCE: &'static str = r#"
        declare register X(7:0)
        X <- X;
        X <- X(7:0);
    "#;

    let vhdl = util::compile(SOURCE);
    assert_eq!(vhdl.signals().control_signals.len(), 1);
}

#[test]
fn different_ctx_size() {
    const SOURCE: &'static str = r#"
        declare register X(7:0)
        X <- X(3:0) + X(3:0) = "0000";
        X <- X(3:0) + X(3:0) = "00000";
    "#;

    let vhdl = util::compile(SOURCE);
    assert_eq!(vhdl.signals().control_signals.len(), 2);
}
