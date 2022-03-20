use compiler_backend_vhdl::BackendVhdl;
use rt_easy_vhdl::Vhdl;

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
