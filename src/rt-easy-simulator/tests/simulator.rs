mod util;

use rt_easy_simulator::Simulator;
use rtcore::{common::Span, value::Value};
use rtprogram::Ident;

const SOURCE: &'static str = r#"
    declare register X(7:0), Y(2:0)

    X <- 2, Y <- 3;
    X <- Y + 4, Y <- X(1);

    END: nop, nop, if 0 then goto END fi;
"#;

#[test]
fn simulator_reset() {
    let mut simulator = Simulator::init(util::compile(SOURCE));

    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_dec("2").unwrap()
    );
    assert_eq!(
        simulator.register_value(&Ident("Y".to_string())).unwrap(),
        Value::parse_dec("3").unwrap()
    );

    simulator.reset(true);
    assert_eq!(simulator.cycle_count(), 0);
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_dec("0").unwrap()
    );
    assert_eq!(
        simulator.register_value(&Ident("Y".to_string())).unwrap(),
        Value::parse_dec("0").unwrap()
    );
}

#[test]
fn simulator_cycle_count() {
    let mut simulator = Simulator::init(util::compile(SOURCE));

    while !simulator.is_finished() {
        simulator.step(false).unwrap();
    }

    assert_eq!(simulator.cycle_count(), 3);
}

#[test]
fn simulator_statement_span() {
    let simulator = Simulator::init(util::compile(SOURCE));

    assert_eq!(simulator.statement_span(0), Some(Span { start: 42, end: 56 }));
    assert_eq!(simulator.statement_span(1), Some(Span { start: 62, end: 83 }));
    assert_eq!(simulator.statement_span(3), None);
}
