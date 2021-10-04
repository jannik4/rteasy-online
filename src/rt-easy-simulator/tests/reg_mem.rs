mod util;

use rt_easy_simulator::Simulator;
use rtcore::{program::Ident, value::Value};

#[test]
fn dr_then_write() {
    const SOURCE: &'static str = r#"
    declare register AR(3:0), DR(7:0)
    declare memory MEM(AR,DR)

    AR <- 0, DR <- 17;
    DR <- 5, write MEM; # This should write 17 not 5
    read MEM; # This should read 17
    "#;

    let mut simulator = Simulator::init(util::compile(SOURCE));
    while !simulator.is_finished() {
        simulator.step(false).unwrap();
    }

    assert_eq!(
        simulator.register_value(&Ident("DR".to_string())).unwrap(),
        Value::parse_dec("17").unwrap()
    );
}

#[test]
fn ar_then_read() {
    const SOURCE: &'static str = r#"
    declare register AR(3:0), DR(7:0)
    declare memory MEM(AR,DR)

    AR <- 0, DR <- 17;
    write MEM;
    DR <- 0;

    AR <- 1, read MEM; # This should read at address 0 not 1 => DR should be 17
    "#;

    let mut simulator = Simulator::init(util::compile(SOURCE));
    while !simulator.is_finished() {
        simulator.step(false).unwrap();
    }

    assert_eq!(
        simulator.register_value(&Ident("DR".to_string())).unwrap(),
        Value::parse_dec("17").unwrap()
    );
}

#[test]
fn read_then_read() {
    const SOURCE: &'static str = r#"
    declare register AR(3:0), DR(3:0)
    declare memory MEM(AR,DR)
    declare memory MEM_R(DR,AR)

    # Setup mem
    AR <- 0, DR <- 1; write MEM;
    DR <- 0, AR <- 5; write MEM_R;
    DR <- 1, AR <- 7; write MEM_R;

    # MEM  : 0 => 1
    # MEM_R: 0 => 5, 1 => 7

    # Reset AR/DR
    AR <- 0, DR <- 0;

    # Read
    read MEM,   # After tact: DR <- 1
    read MEM_R; # After tact: AR <- 5
    "#;

    let mut simulator = Simulator::init(util::compile(SOURCE));
    while !simulator.is_finished() {
        simulator.step(false).unwrap();
    }

    assert_eq!(
        simulator.register_value(&Ident("AR".to_string())).unwrap(),
        Value::parse_dec("5").unwrap()
    );
    assert_eq!(
        simulator.register_value(&Ident("DR".to_string())).unwrap(),
        Value::parse_dec("1").unwrap()
    );
}
