mod util;

use rt_easy_simulator::Simulator;
use rtcore::{program::Ident, value::Value};

#[test]
fn mul_sm() {
    const SOURCE: &'static str = r#"
        declare input IN(7:0)
        declare output OUT(7:0)
        declare register A(7:0), Q(7:0), M(7:0), COUNT(2:0)

        INIT:
            A <- 0, COUNT <- 0,
            M <- IN;
            Q <- IN;

        ADD:
            if Q(0) = 1 then
                A(7:0) <- A(6:0) + M(6:0)
            else
                A(7:0) <- A(6:0) + 0
            fi;

        RSHIFT_AND_TEST:
            A(7) <- 0, A(6:0).Q <- A.Q(7:1),
            if COUNT <> 6 then
                COUNT <- COUNT + 1, goto ADD
            fi;

        SIGN:
            A(7) <- M(7) xor Q(0), A(6:0).Q <- A.Q(7:1);
        OUTPUT:
            OUT <- Q;
            OUT <- A;
    "#;

    let mut simulator = Simulator::init(util::compile(SOURCE));

    // 12 * 7
    simulator.write_bus(&Ident("IN".to_string()), Value::parse_dec("12").unwrap()).unwrap();
    simulator.step(false).unwrap();
    simulator.write_bus(&Ident("IN".to_string()), Value::parse_dec("7").unwrap()).unwrap();
    while !simulator.is_finished() {
        simulator.step(false).unwrap();
    }
    assert_eq!(
        simulator.register_value(&Ident("Q".to_string())).unwrap(),
        Value::parse_dec("84").unwrap()
    );
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_dec("0").unwrap()
    );

    simulator.reset();

    // 31 * 47
    simulator.write_bus(&Ident("IN".to_string()), Value::parse_dec("31").unwrap()).unwrap();
    simulator.step(false).unwrap();
    simulator.write_bus(&Ident("IN".to_string()), Value::parse_dec("47").unwrap()).unwrap();
    while !simulator.is_finished() {
        simulator.step(false).unwrap();
    }
    assert_eq!(
        simulator.register_value(&Ident("Q".to_string())).unwrap(),
        Value::parse_dec("177").unwrap()
    );
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_dec("5").unwrap()
    );
}
