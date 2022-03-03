use std::{fs, path::PathBuf};

use rt_easy_compiler_backend_vhdl::{Args, BackendVhdl, Vhdl};

fn main() {
    let vhdl = compile(SOURCE);

    let path = path("simple.vhdl");
    fs::write(&path, vhdl.render().unwrap()).unwrap();

    println!("Saved vhdl code to: {:?}", path);
}

const SOURCE: &'static str = r#"
declare input IN(7:0)
declare output OUT_A, OUT_B(7:0)
declare register X(7:0), Y
declare bus B(7:0), B_2

INPUT:
    X <- IN, Y <- 1;

LOGIC:
    X <- B + B, B <- X, Y <- 0, if Y then goto OUTPUT fi;

LOOP:
    if 1 then B_2 <- 1 fi, goto LOOP;

OUTPUT:
    OUT_A <- 1, OUT_B <- X;
"#;

fn compile(source: &str) -> Vhdl {
    let ast = match parser::parse(source) {
        Ok(ast) => ast,
        Err(e) => panic!("{}", parser::pretty_print_error(&e, source, None, false)),
    };

    let backend = BackendVhdl;
    match compiler::compile(
        &backend,
        Args { module_name: "simple".to_string() },
        ast,
        &Default::default(),
    ) {
        Ok(vhdl) => vhdl,
        Err(e) => panic!("{}", e.pretty_print(source, None, false)),
    }
}

fn path(file_name: &str) -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "examples", "vhdl", file_name].iter().collect()
}
