mod util;

use rt_easy_simulator::Simulator;
use rtcore::{program::Ident, value::Value};

#[test]
fn switch_case() {
    const SOURCE: &'static str = r#"
        declare register X(7:0)
        declare bus B(7:0), C(7:0)

        SW1:
        switch X {
            case 0 + 0x0  : X <- X + 1, goto SW1
            case 0b1 + "0": if 0 then nop else X <- X + 5, goto SW1 fi
            default       : nop, X <- X - 2
        };
        # --- 1 ---

        SW2:
        switch B(2:0).C(0) {
            case "0110": X <- 42
            default    : X <- 1
        }, if 1 then B <- 0b11 fi;
        # --- 2 ---
    "#;

    let mut simulator = Simulator::init(util::compile(SOURCE));

    // 1
    for _ in 0..3 {
        simulator.step(false).unwrap();
    }
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_dec("4").unwrap()
    );

    // 2
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_dec("42").unwrap()
    );
}

#[test]
fn switch_case_const_expr() {
    const SOURCE: &'static str = r#"
        declare register X(7:0)

        SW:
        switch X {
            case 1 = 0: # 0
                X(0) <- 1, goto SW
            case 1 <> 0: # 1
                X(1) <- 1, goto SW
            case (1 < 0) + (1 <= 3) + (2 > 0) + ("1" >= 0x1): # 11
                X(2) <- 1, goto SW
            case 8 - 1: # 111
                X(3) <- 1, goto SW
            case "11"."11": # 1111
                X(4) <- 1, goto SW
            case (((1 and 1) or 0b11) xor 0b11100): # 11111
                X(5) <- 1, goto SW
            default:
                X(7:6) <- "11"
        };
    "#;

    let mut simulator = Simulator::init(util::compile(SOURCE));

    // Run to end
    while !simulator.is_finished() {
        simulator.step(false).unwrap();
    }

    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_bin("11111111").unwrap()
    );
}
