use rt_easy::compiler::Error;
use rt_easy::rtcore::program::Program;

#[allow(dead_code)] // Not used by every test file
pub fn compile(source: &str) -> Program {
    match compile_(source) {
        Ok(program) => program,
        Err(e) => panic!("{:#?}", e),
    }
}

#[allow(dead_code)] // Not used by every test file
pub fn compile_error(source: &str) -> Error {
    match compile_(source) {
        Ok(_) => panic!("Expected error"),
        Err(e) => e,
    }
}

fn compile_(source: &str) -> Result<Program, Error> {
    let ast = match rt_easy::parser::parse(source) {
        Ok(ast) => ast,
        Err(e) => panic!("{}", rt_easy::parser::pretty_print_error(&e, source)),
    };

    let backend = rt_easy::compiler_backend_simulator::BackendSimulator;
    rt_easy::compiler::compile(&backend, ast, &Default::default())
}
