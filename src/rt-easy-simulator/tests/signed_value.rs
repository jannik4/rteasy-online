mod util;

use rt_easy_simulator::Simulator;
use rtcore::value::{SignedValue, Value};
use rtprogram::Ident;

const SOURCE: &'static str = r#"
    declare register X(7:0), Y(3:0)
    declare bus B(3:0)
    declare register array ARR(3:0)[4]
    declare memory MEM(X, Y)
"#;

#[test]
fn signed_value() {
    let mut simulator = Simulator::init(util::compile(SOURCE));

    // Register
    simulator
        .write_register(
            &Ident("X".to_string()),
            SignedValue::Negative(Value::parse_dec("2").unwrap()),
        )
        .unwrap();
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_bin("11111110").unwrap()
    );

    // Bus
    simulator
        .write_bus(&Ident("B".to_string()), SignedValue::Negative(Value::parse_bin("001").unwrap()))
        .unwrap();
    assert_eq!(
        simulator.bus_value(&Ident("B".to_string())).unwrap(),
        Value::parse_bin("1111").unwrap()
    );

    // Register array
    simulator
        .write_register_array(
            &Ident("ARR".to_string()),
            3,
            SignedValue::Negative(Value::parse_hex("A").unwrap()),
        )
        .unwrap();
    assert_eq!(
        simulator.register_array_page(&Ident("ARR".to_string()), 1).unwrap()[3].1,
        Value::parse_bin("0110").unwrap()
    );

    // Memory
    simulator
        .write_memory(
            &Ident("MEM".to_string()),
            Value::parse_bin("0").unwrap(),
            SignedValue::Negative(Value::parse_hex("1").unwrap()),
        )
        .unwrap();
    assert_eq!(
        simulator.memory_page(&Ident("MEM".to_string()), Value::parse_dec("1").unwrap()).unwrap()
            [0]
        .1,
        Value::parse_bin("1111").unwrap()
    );
}
