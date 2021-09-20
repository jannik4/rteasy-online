mod util;

use rt_easy::{
    rtcore::{program::Ident, value::Value},
    simulator::Simulator,
};

#[test]
fn input_misc() {
    const SOURCE: &'static str = r#"
declare input IN(3:0)
declare bus BUS(3:0)
declare register REG(3:0)

S0: BUS <- IN; 
S1: nop;
S2: REG <- 0;
S3: REG <- BUS, BUS <- IN;
"#;

    let mut simulator = Simulator::init(util::compile(SOURCE));
    let zero = Value::parse_dec("0").unwrap();
    let three = Value::parse_dec("3").unwrap();
    let six = Value::parse_dec("6").unwrap();

    simulator.write_bus(&Ident("IN".to_string()), three.clone()).unwrap();

    // S0
    simulator.step().unwrap();
    assert_eq!(simulator.bus_value(&Ident("IN".to_string())).unwrap(), three.clone());
    assert_eq!(simulator.bus_value(&Ident("BUS".to_string())).unwrap(), three.clone());
    assert_eq!(simulator.register_value(&Ident("REG".to_string())).unwrap(), zero.clone());

    // S1
    simulator.step().unwrap();
    assert_eq!(simulator.bus_value(&Ident("IN".to_string())).unwrap(), three.clone());
    assert_eq!(simulator.bus_value(&Ident("BUS".to_string())).unwrap(), zero.clone());
    assert_eq!(simulator.register_value(&Ident("REG".to_string())).unwrap(), zero.clone());

    // S2
    simulator.step().unwrap();
    assert_eq!(simulator.bus_value(&Ident("IN".to_string())).unwrap(), three.clone());
    assert_eq!(simulator.bus_value(&Ident("BUS".to_string())).unwrap(), zero.clone());
    assert_eq!(simulator.register_value(&Ident("REG".to_string())).unwrap(), zero.clone());

    simulator.write_bus(&Ident("IN".to_string()), six.clone()).unwrap();

    // S3
    simulator.step().unwrap();
    assert_eq!(simulator.bus_value(&Ident("IN".to_string())).unwrap(), six.clone());
    assert_eq!(simulator.bus_value(&Ident("BUS".to_string())).unwrap(), six.clone());
    assert_eq!(simulator.register_value(&Ident("REG".to_string())).unwrap(), six.clone());
}

#[test]
fn write_to_input() {
    const SOURCES: &'static [&'static str] = &[
        r#"
            declare input IN(3:0)
            declare bus BUS(3:0)
            IN <- 2;
        "#,
        r#"
            declare input IN(3:0)
            declare bus BUS(3:0)
            BUS.IN <- 2;
        "#,
        r#"
            declare input IN(3:0)
            declare bus BUS(3:0)
            if 1 then BUS.IN <- 2 fi;
        "#,
    ];

    for source in SOURCES {
        let _err = util::compile_error(source);
        // TODO: Check err is "cannot assign to input"
    }
}
