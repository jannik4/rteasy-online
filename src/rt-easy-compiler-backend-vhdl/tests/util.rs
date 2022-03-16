use rt_easy_compiler_backend_vhdl::{Args, BackendVhdl, Vhdl};

#[allow(dead_code)] // Not used by every test file
pub fn compile_with_name(source: &str, module_name: impl Into<String>) -> Vhdl {
    let ast = match parser::parse(source) {
        Ok(ast) => ast,
        Err(e) => panic!("{}", parser::pretty_print_error(&e, source, None, false)),
    };

    let backend = BackendVhdl;
    match compiler::compile(
        &backend,
        Args { module_name: module_name.into() },
        ast,
        &Default::default(),
    ) {
        Ok(vhdl) => vhdl,
        Err(e) => panic!("{}", e.pretty_print(source, None, false)),
    }
}

#[allow(dead_code)] // Not used by every test file
pub fn compile(source: &str) -> Vhdl {
    compile_with_name(source, "my_module")
}
