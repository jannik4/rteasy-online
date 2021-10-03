use rt_easy_compiler::{Error, Options};

#[allow(dead_code)] // Not used by every test file
pub fn check(source: &str) {
    check_with_options(source, &Default::default());
}

#[allow(dead_code)] // Not used by every test file
pub fn check_with_options(source: &str, options: &Options) {
    match check_(source, options) {
        Ok(()) => (),
        Err(e) => panic!("{}", e.pretty_print(source, None, false)),
    }
}

#[allow(dead_code)] // Not used by every test file
pub fn check_err(source: &str) -> Error {
    match check_(source, &Default::default()) {
        Ok(()) => panic!("Expected error"),
        Err(e) => e,
    }
}

fn check_(source: &str, options: &Options) -> Result<(), Error> {
    let ast = match parser::parse(source) {
        Ok(ast) => ast,
        Err(e) => panic!("{}", parser::pretty_print_error(&e, source, false)),
    };

    rt_easy_compiler::check(ast, options)
}
