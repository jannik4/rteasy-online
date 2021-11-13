use rtcore::ast::Ast;

#[allow(dead_code)] // Not used by every test file
pub fn parse(source: &str) -> Ast<'_> {
    match rt_easy_parser::parse(source) {
        Ok(ast) => ast,
        Err(e) => panic!("{}", rt_easy_parser::pretty_print_error(&e, source, None, false)),
    }
}

#[allow(dead_code)] // Not used by every test file
pub fn parse_err(source: &str) -> toktok::Error<rt_easy_parser::Token> {
    match rt_easy_parser::parse(source) {
        Ok(_) => panic!("expected error"),
        Err(e) => e,
    }
}
