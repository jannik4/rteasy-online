mod util;

use rt_easy_simulator::Simulator;
use rtcore::{program::Ident, value::Value};

const SOURCE: &'static str = r#"
declare register X(7:0)

X <- 8 - 2 + 3;
X <- 8 - (2 + 3);

X <- 8 - 2 + 5 - 1;
X <- 8 - (2 + 5) - 1;
"#;

#[test]
fn add_sub_assoc() {
    let mut simulator = Simulator::init(util::compile(SOURCE));

    simulator.step().unwrap();
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_dec("9").unwrap()
    );
    simulator.step().unwrap();
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_dec("3").unwrap()
    );

    simulator.step().unwrap();
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_dec("10").unwrap()
    );
    simulator.step().unwrap();
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_dec("0").unwrap()
    );
}
