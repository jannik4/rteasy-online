use anyhow::{bail, Context, Result};
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

pub fn check(file: PathBuf, ansi_colors: bool) -> Result<()> {
    let source = fs::read_to_string(&file)
        .with_context(|| format!("Failed to read from {}", file.display()))?;
    let file_name = file.file_name().and_then(OsStr::to_str);

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

pub fn test(_file: PathBuf, _test_file: PathBuf, _ansi_colors: bool) -> Result<()> {
    bail!("unimplemented")
}
