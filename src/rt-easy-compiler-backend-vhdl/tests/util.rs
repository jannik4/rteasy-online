use compiler::Error;
use rt_easy_compiler_backend_vhdl::{BackendVhdl, Vhdl};

#[allow(dead_code)] // Not used by every test file
pub fn compile(source: &str) -> Vhdl {
    let ast = match parser::parse(source) {
        Ok(ast) => ast,
        Err(e) => panic!("{}", parser::pretty_print_error(&e, source, None, false)),
    };

    let backend = BackendVhdl;
    match compiler::compile(&backend, (), ast, &Default::default()) {
        Ok(vhdl) => vhdl,
        Err(e) => panic!("{}", e.pretty_print(source, None, false)),
    }
}

#[allow(dead_code)] // Not used by every test file
pub fn compile_err(source: &str) -> Error {
    let ast = match parser::parse(source) {
        Ok(ast) => ast,
        Err(e) => panic!("{}", parser::pretty_print_error(&e, source, None, false)),
    };

    let backend = BackendVhdl;
    match compiler::compile(&backend, (), ast, &Default::default()) {
        Ok(_) => panic!("Expected error"),
        Err(e) => e,
    }
}
