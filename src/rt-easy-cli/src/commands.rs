use anyhow::{bail, Context, Result};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

pub fn check(file: PathBuf, ansi_colors: bool) -> Result<()> {
    let (source, file_name) = read_file(&file)?;

    let ast = match parser::parse(&source) {
        Ok(ast) => ast,
        Err(e) => bail!(parser::pretty_print_error(&e, &source, file_name, ansi_colors)),
    };
    match compiler::check(ast, &Default::default()) {
        Ok(()) => (),
        Err(e) => bail!(e.pretty_print(&source, file_name, ansi_colors)),
    };

    Ok(())
}

pub fn test(file: PathBuf, test_file: PathBuf, ansi_colors: bool) -> Result<()> {
    // Build rt file
    let program = {
        let (source, file_name) = read_file(&file)?;

        let ast = match parser::parse(&source) {
            Ok(ast) => ast,
            Err(e) => bail!(parser::pretty_print_error(&e, &source, file_name, ansi_colors)),
        };

        let backend = compiler_backend_simulator::BackendSimulator;
        match compiler::compile(&backend, (), ast, &Default::default()) {
            Ok(program) => program,
            Err(e) => bail!(e.pretty_print(&source, file_name, ansi_colors)),
        }
    };

    // Parse test file
    let unit_test = {
        let (source, file_name) = read_file(&test_file)?;

        match unit_test::parser::parse(&source) {
            Ok(unit_test) => unit_test,
            Err(e) => {
                bail!(unit_test::parser::pretty_print_error(&e, &source, file_name, ansi_colors))
            }
        }
    };

    // Run unit test
    unit_test::run(program, unit_test).context("Tests failed")?;

    Ok(())
}

fn read_file(file: &Path) -> Result<(String, Option<&str>)> {
    let source = fs::read_to_string(&file)
        .with_context(|| format!("Failed to read from {}", file.display()))?;
    let file_name = file.file_name().and_then(OsStr::to_str);
    Ok((source, file_name))
}
