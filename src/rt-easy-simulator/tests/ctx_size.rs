mod util;

use rt_easy_simulator::Simulator;
use rtcore::{program::Ident, value::Value};

#[test]
fn ctx_size() {
    const SOURCE: &'static str = r#"
        declare register X(7:0)
        declare bus B(7:0), C(7:0)

        X <- (1 > 0) + (X = X); # 1
        X(3:0) <- (not 0) xor "1010"; # 2
        X <- sxt B(0), B <- 1; # 3
    "#;

    let mut simulator = Simulator::init(util::compile(SOURCE));

    // 1
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_bin("10").unwrap()
    );

    // 2
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_bin("0101").unwrap()
    );

    // 3
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_bin("11111111").unwrap()
    );
}
