#![deny(rust_2018_idioms)]

mod ast;
mod generator;
mod lexer;
mod parser;
mod token_map;

// TODO: Use anyhow for errors

use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn process(file: &str) -> Result<()> {
    // Load source code
    let source = source(file)?;

    // Split into parts
    let (parser_source, token_map_source, rust_source) = split_into_parts(&source)?;

    // Build ast
    let ast = match parser::parse(parser_source) {
        Ok(ast) => ast,
        Err(err) => {
            let options =
                toktok::PrettyPrintOptions { source: Some(parser_source), ..Default::default() };
            panic!("{}", err.pretty_print(&options))
        }
    };

    // Build token map
    let token_map = token_map::parse_and_build(token_map_source)?;

    // Generate parser content
    let parser_content = generator::generate(ast, token_map)?;

    // Merge output
    let content = format!(
        r###"mod parser{{
    #![allow(non_snake_case)]
    #![allow(unused_braces)]
    #![allow(clippy::all)]
    #![deny(clippy::correctness)]

    // ------------------------------------------------------------------
    // User code
    // ------------------------------------------------------------------

{}

    // ------------------------------------------------------------------
    // Parser
    // ------------------------------------------------------------------

{}

    // ------------------------------------------------------------------
    // Intern
    // ------------------------------------------------------------------

    #[allow(unused)]
    use ::toktok::Parser as _;
    mod __intern__ {{
        pub use ::toktok::{{combinator as c, State, Input, PResult, Parser, Error, ParserError}};
    }}
}}
    "###,
        textwrap::indent(&rust_source, "    "),
        textwrap::indent(&parser_content, "    ")
    );

    write(file, &content)?;
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/{}", file);

    Ok(())
}

fn source(file: &str) -> Result<String> {
    let root = env::var("CARGO_MANIFEST_DIR")?;
    let path = Path::new(&root).join("src").join(file);
    let mut file = File::open(path)?;

    let mut source = String::new();
    file.read_to_string(&mut source)?;

    Ok(source)
}

fn split_into_parts(source: &str) -> Result<(&str, &str, &str)> {
    let mut parts = source.split("+++");

    let part0 = parts.next().ok_or("expected 3 parts")?;
    let part1 = parts.next().ok_or("expected 3 parts")?;
    let part2 = parts.next().ok_or("expected 3 parts")?;

    Ok((part0, part1, part2))
}

fn write(file: &str, content: &str) -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").ok_or("could not find OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join(format!("{}.rs", file));
    fs::write(&dest_path, content)?;

    Ok(())
}
