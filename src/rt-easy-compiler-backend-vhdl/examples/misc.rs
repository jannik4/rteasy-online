use std::{fs, path::PathBuf};

use rt_easy_compiler_backend_vhdl::{BackendVhdl, Vhdl};

fn main() {
    let vhdl = compile(SOURCE);

    let path = path("misc.vhdl");
    fs::write(&path, vhdl.render("misc").unwrap()).unwrap();

    println!("Saved vhdl code to: {:?}", path);
}

const SOURCE: &'static str = r#"
declare register A(3:0), B(3:0), C(7:0), D(0:12), XX
declare bus BUS(7:0)

if A + B = 2 then nop, A <- A + B fi;
MY_LABEL: if ((A and B + 0b11) or not "11110000") > 0 then goto END fi;

if A(0) then
    goto END, BUS <- 3
else
    if A(1) then
        goto MY_LABEL, BUS <- 3
    else
        goto END, A <- A = B
    fi
fi;

A <- A + B;
A <- A = B;
B <- (sxt 1) + -A;
C <- (A and B + 0b11) or not "11110000";

BUS <- 3, C <- BUS;

D <- D(1:3) + A(2) + B(2:1);

END:

"#;

fn compile(source: &str) -> Vhdl {
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

fn path(file_name: &str) -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "examples", "vhdl", file_name].iter().collect()
}
