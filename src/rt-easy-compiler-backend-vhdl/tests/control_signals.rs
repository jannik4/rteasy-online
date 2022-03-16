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
fn duplicate_number_different_kind() {
    const SOURCE: &'static str = r#"
        declare register X(7:0)
        X <- "1100";
        X <- 0b1100; X <- 0B001100; X <- %0000000000000000001100;
        X <- 12; X <- 012;
        X <- 0xc; X <- 0X000C; X <- $00000c;
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
