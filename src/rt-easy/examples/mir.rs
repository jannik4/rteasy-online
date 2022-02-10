const SOURCE: &'static str = r#"
declare bus      A(3:0), B(3:0)
declare register X(3:0), Y(3:0)

if A(0) then B(1) <- 1, if 1 then Y <- 1 fi else Y <- 7, goto END fi,
if X(0) then X <- A + Y fi,
A(1) <- B(1), 
A(0) <- 1;

END:
goto END;
"#;

fn main() {
    let ast = match rt_easy::parser::parse(SOURCE) {
        Ok(ast) => ast,
        Err(e) => panic!("{}", rt_easy::parser::pretty_print_error(&e, SOURCE, None, false)),
    };

    let backend = rt_easy::compiler_backend_simulator::BackendSimulator;
    let options = rt_easy::compiler::Options { print_mir_unordered: true, print_mir: true };
    match rt_easy::compiler::compile(&backend, (), ast, &options) {
        Ok(_program) => (),
        Err(e) => panic!("{:#?}", e),
    }
}
