mod util;

use rt_easy_simulator::Simulator;
use rtcore::value::Value;
use rtprogram::Ident;

const SOURCE: &'static str = r#"
declare register A(31:0)

A <- 0b1110 xor 0b0101;
A <- 0b1110 or 0b0101;
A <- 0b1110 nor 0b0101;
A <- 0b1110 and 0b0101;
A <- 0b1110 nand 0b0101;
"#;

#[test]
fn bit_op() {
    let mut simulator = Simulator::init(util::compile(SOURCE));

    // xor
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_bin("1011").unwrap()
    );

    // or
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_bin("1111").unwrap()
    );

    // nor
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_bin("11111111111111111111111111110000").unwrap()
    );

    // and
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_bin("100").unwrap()
    );

    // nand
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_bin("11111111111111111111111111111011").unwrap()
    );
}
