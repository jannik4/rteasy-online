mod util;

use rt_easy_simulator::{Simulator, StepResult, StepResultKind};
use rtcore::{program::Ident, value::Value};

#[test]
fn breakpoints_add_remove() {
    const SOURCE: &'static str = r#"
        declare register A(7:0), Q(7:0)

        INIT: A <- 0; Q <- 0;
        A <- 2 | if A(0) then goto INIT fi;
        nop;
        nop;
        A <- Q, Q <- A;
    "#;

    let mut simulator = Simulator::init(util::compile(SOURCE));

    assert!(simulator.breakpoints().collect::<Vec<_>>().is_empty());

    simulator.add_breakpoint(6);
    simulator.add_breakpoint(7);
    assert!(simulator.breakpoints().collect::<Vec<_>>().is_empty());

    simulator.add_breakpoint(2);
    assert!(simulator.breakpoints().any(|b| b == 2));

    simulator.add_breakpoint(2);
    assert_eq!(simulator.breakpoints().collect::<Vec<_>>(), vec![2]);

    simulator.add_breakpoint(1);
    assert!(simulator.breakpoints().any(|b| b == 1));
    assert!(simulator.breakpoints().any(|b| b == 2));

    simulator.remove_breakpoint(2);
    assert_eq!(simulator.breakpoints().collect::<Vec<_>>(), vec![1]);
}

#[test]
fn breakpoints_stop() {
    const SOURCE: &'static str = r#"
        declare register A(7:0)
        declare bus Q(7:0)

        A <- 1, nop;
        Q <- 7;
        A <- 9, Q <- 2;
    "#;

    let mut simulator = Simulator::init(util::compile(SOURCE));
    simulator.add_breakpoint(2);

    while !simulator.is_finished() {
        let step_result = simulator.step(true).unwrap();
        if matches!(step_result, Some(StepResult { kind: StepResultKind::Breakpoint, .. })) {
            break;
        }
    }

    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_dec("1").unwrap()
    );
    assert_eq!(
        simulator.bus_value(&Ident("Q".to_string())).unwrap(),
        Value::parse_dec("0").unwrap()
    );

    simulator.step(true).unwrap();

    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_dec("9").unwrap()
    );
    assert_eq!(
        simulator.bus_value(&Ident("Q".to_string())).unwrap(),
        Value::parse_dec("2").unwrap()
    );
}
