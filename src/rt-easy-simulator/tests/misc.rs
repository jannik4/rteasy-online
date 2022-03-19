mod util;

use rt_easy_simulator::Simulator;
use rtcore::value::Value;
use rtprogram::Ident;

const SOURCE: &'static str = r#"
declare register A(3:0), B(3:0), C(7:0)

A <- 0b1001, B <- 0b0101;
C <- sxt A.A(3).A(3).A(1); # -------- 1 --------

A <- 8, B <- -1;
C <- A + sxt B; # -------- 2 --------

A <- 0b1111;
C <- A; # -------- 3 --------
C <- sxt A; # -------- 4 --------
C <- not A; # -------- 6 --------
C <- not 0b1111; # -------- 7 --------
C <- not not A; # -------- 8 --------

A(0).A(3:2).A(1) <- 0b1110; # -------- 9 --------

A(3:0) <- sxt  0b1; # -------- 10 --------
A(3:0) <- sxt 0b01; # -------- 11 --------
A <- 0b111 + 0b1; # -------- 12 --------
A <- sxt 0b111 + 0b1; # -------- 13 --------
"#;

#[test]
fn misc() {
    let mut simulator = Simulator::init(util::compile(SOURCE));

    // -------- 1 --------
    simulator.step(false).unwrap();
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("C".to_string())).unwrap(),
        Value::parse_bin("11001110").unwrap()
    );

    // -------- 2 --------
    simulator.step(false).unwrap();
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("C".to_string())).unwrap(),
        Value::parse_dec("7").unwrap()
    );

    // -------- 3 --------
    simulator.step(false).unwrap();
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("C".to_string())).unwrap(),
        Value::parse_bin("1111").unwrap()
    );

    // -------- 4 --------
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("C".to_string())).unwrap(),
        Value::parse_bin("11111111").unwrap()
    );

    // -------- 6 --------
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("C".to_string())).unwrap(),
        Value::parse_bin("11110000").unwrap()
    );

    // -------- 7 --------
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("C".to_string())).unwrap(),
        Value::parse_bin("11110000").unwrap()
    );

    // -------- 8 --------
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("C".to_string())).unwrap(),
        Value::parse_bin("1111").unwrap()
    );

    // -------- 9 --------
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_bin("1101").unwrap()
    );

    // -------- 10 --------
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_bin("1111").unwrap()
    );

    // -------- 11 --------
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_bin("1111").unwrap()
    );

    // -------- 12 --------
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_bin("1000").unwrap()
    );

    // -------- 13 --------
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_bin("0").unwrap()
    );

    simulator.step(false).unwrap();
    assert!(simulator.is_finished());
}
