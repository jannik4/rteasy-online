use rt_easy::{
    rtcore::{
        program::{Bus, Ident},
        value::Value,
    },
    simulator::Simulator,
};

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
    let mut simulator = compile(SOURCE);

    // A
    simulator
        .write_into_bus(
            &Bus { ident: Ident("INBUS".to_string()), range: None },
            Value::parse_dec("3").unwrap(),
        )
        .unwrap();
    simulator.step().unwrap();

    // FACTOR
    simulator
        .write_into_bus(
            &Bus { ident: Ident("INBUS".to_string()), range: None },
            Value::parse_dec("7").unwrap(),
        )
        .unwrap();
    simulator.step().unwrap();

    // Run to end
    while !simulator.is_finished() {
        simulator.step().unwrap();
    }

    assert_eq!(
        simulator.state().read_register(&Ident("RES".to_string()), None).unwrap(),
        Value::parse_dec("21").unwrap()
    );
}

fn compile(source: &str) -> Simulator {
    let ast = match rt_easy::parser::parse(source) {
        Ok(ast) => ast,
        Err(e) => panic!("{}", rt_easy::parser::pretty_print_error(&e, source)),
    };

    let backend = rt_easy::compiler_backend_simulator::BackendSimulator;
    match rt_easy::compiler::compile(&backend, ast, &Default::default()) {
        Ok(program) => Simulator::init(program),
        Err(e) => panic!("{:#?}", e),
    }
}
