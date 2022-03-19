mod util;

use rt_easy_simulator::Simulator;
use rtcore::value::Value;
use rtprogram::Ident;

const SOURCE: &'static str = r#"
declare register A(7:0), FACTOR(7:0), RES(7:0)
declare bus INBUS(7:0), OUTBUS(7:0)

BEGIN:
    A <- INBUS, RES <- 0;
    FACTOR <- INBUS;
LOOP:
    if FACTOR <> 0 then
        RES <- RES + A, FACTOR <- FACTOR - 1, goto LOOP
    else
        OUTBUS <- RES
    fi;
"#;

#[test]
fn mult() {
    let mut simulator = Simulator::init(util::compile(SOURCE));

    // A
    simulator.write_bus(&Ident("INBUS".to_string()), Value::parse_dec("3").unwrap()).unwrap();
    simulator.step(false).unwrap();

    // FACTOR
    simulator.write_bus(&Ident("INBUS".to_string()), Value::parse_dec("7").unwrap()).unwrap();
    simulator.step(false).unwrap();

    // Run to end
    while !simulator.is_finished() {
        simulator.step(false).unwrap();
    }

    assert_eq!(
        simulator.register_value(&Ident("RES".to_string())).unwrap(),
        Value::parse_dec("21").unwrap()
    );
}
