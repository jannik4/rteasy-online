use rtcore::program::Program;

pub fn compile(source: &str) -> Program {
    let ast = match parser::parse(source) {
        Ok(ast) => ast,
        Err(e) => panic!("{}", parser::pretty_print_error(&e, source, None, false)),
    };

    let backend = compiler_backend_simulator::BackendSimulator;
    match compiler::compile(&backend, (), ast, &Default::default()) {
        Ok(program) => program,
        Err(e) => panic!("{}", e.pretty_print(source, None, false)),
    }
}
