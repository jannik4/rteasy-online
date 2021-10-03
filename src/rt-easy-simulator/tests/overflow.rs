mod util;

use rt_easy_simulator::Simulator;
use rtcore::{program::Ident, value::Value};

const SOURCE: &'static str = r#"
declare bus A(3:0), B(1:0)

A <- 15 + 1, B <- 2 + 3; # 1
B <- 3, A <- B + B + B + B + B + B; # 2
A <- 5 - 7; # 3
"#;

#[test]
fn overflow() {
    let mut simulator = Simulator::init(util::compile(SOURCE));

    // 1
    simulator.step().unwrap();
    assert_eq!(
        simulator.bus_value(&Ident("A".to_string())).unwrap(),
        Value::parse_dec("0").unwrap()
    );
    assert_eq!(
        simulator.bus_value(&Ident("B".to_string())).unwrap(),
        Value::parse_dec("1").unwrap()
    );

    // 2
    simulator.step().unwrap();
    assert_eq!(
        simulator.bus_value(&Ident("A".to_string())).unwrap(),
        Value::parse_dec("2").unwrap()
    );

    // 3
    simulator.step().unwrap();
    assert_eq!(
        simulator.bus_value(&Ident("A".to_string())).unwrap(),
        Value::parse_dec("14").unwrap()
    );
}
