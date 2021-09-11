use rt_easy::{
    rtcore::{program::Ident, value::Value},
    simulator::Simulator,
};

const SOURCE: &'static str = r#"
declare bus A(3:0), B(1:0)

A <- 15 + 1, B <- 2 + 3; # 1
B <- 3, A <- B + B + B + B + B + B; # 2
A <- 5 - 7; # 3
"#;

#[test]
fn overflow() {
    let mut simulator = compile(SOURCE);

    // 1
    simulator.step().unwrap();
    assert_eq!(
        simulator.bus_value(&Ident("A".to_string())).unwrap(),
        Value::parse_dec("0").unwrap()
    );
    assert_eq!(
        simulator.bus_value(&Ident("B".to_string())).unwrap(),
        Value::parse_dec("1").unwrap()
    );

    // 2
    simulator.step().unwrap();
    assert_eq!(
        simulator.bus_value(&Ident("A".to_string())).unwrap(),
        Value::parse_dec("2").unwrap()
    );

    // 3
    simulator.step().unwrap();
    assert_eq!(
        simulator.bus_value(&Ident("A".to_string())).unwrap(),
        Value::parse_dec("14").unwrap()
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
