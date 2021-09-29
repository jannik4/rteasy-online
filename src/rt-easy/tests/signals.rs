mod util;

#[test]
fn signals_misc() {
    const SOURCE: &'static str = r#"
        declare register X(7:0), Y(7:0)
        declare bus B(7:0), C(7:0)

        # Assert ok
        X <- 12 + 5;
        assert X = 17;

        B <- 0, X <- 12 + 5;

        X <- B, B <- -1;
        assert X = 0b11111111;

        # Switch
        SW:
        switch Y {
            case 0: Y <- Y + 1, goto SW
            case 1 and 1: Y <- Y + 1, goto SW
            case 1 + 1: Y <- Y + 1, goto SW
            case 3: if 0 then nop else nop fi
            default: nop, nop
        };

        # Assert err
        X <- 1;
        assert X + 2 = 4;

        # Unreachable
        nop;
    "#;

    let program = util::compile(SOURCE);
    let signals = program.signals();

    assert_eq!(
        signals.condition_signals,
        vec!["Y = 0", "Y = (1 and 1)", "Y = 1 + 1", "Y = 3", "0"]
    );
    assert_eq!(
        signals.control_signals,
        vec!["X <- 12 + 5", "B <- 0", "B <- -1", "X <- B", "Y <- Y + 1", "X <- 1"]
    );
}

#[test]
fn signals_no_duplicates() {
    const SOURCE: &'static str = r#"
        declare register X(7:0), Y(7:0)
        declare bus B(7:0), C(7:0)

        if B + C = 1 then nop fi;
        if X(0) then nop fi;
        if B + C = 1 then nop fi; # duplicate

        X <- 12 + (5 and 1);
        Y <- 7;
        X <- 12 + (5 and 1); # duplicate
    "#;

    let program = util::compile(SOURCE);
    let signals = program.signals();

    assert_eq!(signals.condition_signals, vec!["B + C = 1", "X(0)"]);
    assert_eq!(signals.control_signals, vec!["X <- 12 + (5 and 1)", "Y <- 7",]);
}

#[test]
fn signals_bus_ordering() {
    const SOURCE: &'static str = r#"
        declare register X(7:0), Y(7:0)
        declare bus B(7:0), C(7:0)

        X <- B, B <- 1;
    "#;

    let program = util::compile(SOURCE);
    let signals = program.signals();

    assert!(signals.condition_signals.is_empty());
    assert_eq!(signals.control_signals, vec!["B <- 1", "X <- B",]);
}

#[test]
fn signals_no_nop_goto_assert() {
    const SOURCE: &'static str = r#"
        declare register X(7:0), Y(7:0)
        declare bus B(7:0), C(7:0)

        START: nop; goto START; assert 1;
    "#;

    let program = util::compile(SOURCE);
    let signals = program.signals();

    assert!(signals.condition_signals.is_empty());
    assert!(signals.control_signals.is_empty());
}
