use rt_easy::{
    rtcore::{program::Ident, value::Value},
    simulator::Simulator,
};

#[test]
fn dr_then_write() {
    const SOURCE: &'static str = r#"
    declare register AR(3:0), DR(7:0)
    declare memory MEM(AR,DR)

    AR <- 0, DR <- 17;
    DR <- 5, write MEM; # This should write 17 not 5
    read MEM; # This should read 17
    "#;

    let mut simulator = compile(SOURCE);
    while !simulator.is_finished() {
        simulator.step().unwrap();
    }

    assert_eq!(
        simulator.state().read_register(&Ident("DR".to_string()), None).unwrap(),
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

    let mut simulator = compile(SOURCE);
    while !simulator.is_finished() {
        simulator.step().unwrap();
    }

    assert_eq!(
        simulator.state().read_register(&Ident("DR".to_string()), None).unwrap(),
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

    let mut simulator = compile(SOURCE);
    while !simulator.is_finished() {
        simulator.step().unwrap();
    }

    assert_eq!(
        simulator.state().read_register(&Ident("AR".to_string()), None).unwrap(),
        Value::parse_dec("5").unwrap()
    );
    assert_eq!(
        simulator.state().read_register(&Ident("DR".to_string()), None).unwrap(),
        Value::parse_dec("1").unwrap()
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
