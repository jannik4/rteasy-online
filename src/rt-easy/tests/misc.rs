use rt_easy::{
    rtcore::{program::Ident, value::Value},
    simulator::Simulator,
};

const SOURCE: &'static str = r#"
declare register A(3:0), B(3:0), C(7:0)

A <- 0b1001, B <- 0b0101;
C <- sxt A.A(3).A(3).A(1); # -------- 1 --------

A <- 8, B <- -1;
C <- A + sxt B; # -------- 2 --------

A <- 0b1111;
C <- A; # -------- 3 --------
C <- sxt A; # -------- 4 --------
C <- not A; # -------- 6 --------
C <- not 0b1111; # -------- 7 --------
C <- not not A; # -------- 8 --------

A(0).A(3:2).A(1) <- 0b1110; # -------- 9 --------

A(3:0) <- sxt  0b1; # -------- 10 --------
A(3:0) <- sxt 0b01; # -------- 11 --------
A <- 0b111 + 0b1; # -------- 12 --------
A <- sxt 0b111 + 0b1; # -------- 13 --------
"#;

#[test]
fn misc() {
    let mut simulator = compile(SOURCE);

    // -------- 1 --------
    simulator.step().unwrap();
    simulator.step().unwrap();
    assert_eq!(
        simulator.state().read_register(&Ident("C".to_string()), None).unwrap(),
        Value::parse_bin("11001110", false).unwrap()
    );

    // -------- 2 --------
    simulator.step().unwrap();
    simulator.step().unwrap();
    assert_eq!(
        simulator.state().read_register(&Ident("C".to_string()), None).unwrap(),
        Value::parse_dec("7").unwrap()
    );

    // -------- 3 --------
    simulator.step().unwrap();
    simulator.step().unwrap();
    assert_eq!(
        simulator.state().read_register(&Ident("C".to_string()), None).unwrap(),
        Value::parse_bin("1111", false).unwrap()
    );

    // -------- 4 --------
    simulator.step().unwrap();
    assert_eq!(
        simulator.state().read_register(&Ident("C".to_string()), None).unwrap(),
        Value::parse_bin("11111111", false).unwrap()
    );

    // -------- 6 --------
    simulator.step().unwrap();
    assert_eq!(
        simulator.state().read_register(&Ident("C".to_string()), None).unwrap(),
        Value::parse_bin("11110000", false).unwrap()
    );

    // -------- 7 --------
    simulator.step().unwrap();
    assert_eq!(
        simulator.state().read_register(&Ident("C".to_string()), None).unwrap(),
        Value::parse_bin("11110000", false).unwrap()
    );

    // -------- 8 --------
    simulator.step().unwrap();
    assert_eq!(
        simulator.state().read_register(&Ident("C".to_string()), None).unwrap(),
        Value::parse_bin("1111", false).unwrap()
    );

    // -------- 9 --------
    simulator.step().unwrap();
    assert_eq!(
        simulator.state().read_register(&Ident("A".to_string()), None).unwrap(),
        Value::parse_bin("1101", false).unwrap()
    );

    // -------- 10 --------
    simulator.step().unwrap();
    assert_eq!(
        simulator.state().read_register(&Ident("A".to_string()), None).unwrap(),
        Value::parse_bin("1111", false).unwrap()
    );

    // -------- 11 --------
    simulator.step().unwrap();
    assert_eq!(
        simulator.state().read_register(&Ident("A".to_string()), None).unwrap(),
        Value::parse_bin("1111", false).unwrap()
    );

    // -------- 12 --------
    simulator.step().unwrap();
    assert_eq!(
        simulator.state().read_register(&Ident("A".to_string()), None).unwrap(),
        Value::parse_bin("1000", false).unwrap()
    );

    // -------- 13 --------
    simulator.step().unwrap();
    assert_eq!(
        simulator.state().read_register(&Ident("A".to_string()), None).unwrap(),
        Value::parse_bin("0", false).unwrap()
    );

    simulator.step().unwrap();
    assert!(simulator.is_finished());
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
