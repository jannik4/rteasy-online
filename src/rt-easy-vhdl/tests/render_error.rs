mod util;

use memory_file::MemoryFile;
use rt_easy_vhdl::{error::RenderError, Ident};
use std::collections::HashMap;

#[test]
fn memory_not_found() {
    const SOURCE: &'static str = r#"
        declare register X, Y
        declare memory MEM(X, Y)

        X <- 1;
    "#;

    let vhdl = util::compile(SOURCE);
    let memories = HashMap::from([(Ident("MEM_NOT_FOUND".to_string()), MemoryFile::empty(2, 4))]);

    match vhdl.render("my_module", memories) {
        Ok(_) => panic!("expected error"),
        Err(err) => match err {
            RenderError::MemoryNotFound(name) => assert_eq!(name.0, "MEM_NOT_FOUND"),
            _ => panic!("expected RenderError::MemoryNotFound"),
        },
    }
}

#[test]
fn invalid_memory_size() {
    const SOURCE: &'static str = r#"
        declare register X(3:1), Y(2:5)
        declare memory MEM(X, Y)

        X <- 1;
    "#;

    let vhdl = util::compile(SOURCE);
    let memories = HashMap::from([(Ident("MEM".to_string()), MemoryFile::empty(2, 5))]);

    match vhdl.render("my_module", memories) {
        Ok(_) => panic!("expected error"),
        Err(err) => match err {
            RenderError::InvalidMemorySize { name, expected, actual } => {
                assert_eq!(name.0, "MEM");
                assert_eq!(expected, (3, 4));
                assert_eq!(actual, (2, 5));
            }
            _ => panic!("expected RenderError::InvalidMemorySize"),
        },
    }
}
