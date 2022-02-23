use anyhow::{bail, ensure, Context, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

#[derive(Debug)]
struct Code {
    path: PathBuf,
    source: String,
    kind: CodeKind,
}

#[derive(Debug, PartialEq, Eq)]
enum CodeKind {
    Default,
    Ignore,
    NoRun,
    ShouldFail,
    CompileFail,
}

#[derive(Debug)]
enum TestSuccess {
    Passed,
    Ignored,
}

#[derive(Debug)]
struct TestError {
    path: PathBuf,
    source: String,
    error: Error,
}

fn main() {
    let test_results = match run() {
        Ok(test_results) => test_results,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let mut passed = 0;
    let mut ignored = 0;
    let mut failed = Vec::new();
    for test_result in test_results {
        match test_result {
            Ok(TestSuccess::Passed) => passed += 1,
            Ok(TestSuccess::Ignored) => ignored += 1,
            Err(err) => failed.push(err),
        }
    }

    if failed.len() == 0 {
        println!("PASSED\n\n{} passed; {} ignored; {} failed", passed, ignored, failed.len());
    } else {
        for err in &failed {
            eprintln!("Path:\n{:?}\n", err.path);
            eprintln!("Source:\n{}\n", err.source);
            eprintln!("Error:\n{:?}", err.error);
            eprintln!("\n\n--------------------------------\n\n");
        }

        eprintln!("FAILED\n\n{} passed; {} ignored; {} failed", passed, ignored, failed.len());

        process::exit(1);
    }
}

fn run() -> Result<Vec<Result<TestSuccess, TestError>>> {
    let mut test_results = Vec::new();

    for (path, content) in source_files()? {
        let codes = get_code_blocks(&path, &content, |e| test_results.push(Err(e)));
        for code in codes {
            test_results.push(test_code(&code).map_err(|error| TestError {
                path: path.clone(),
                source: code.source.clone(),
                error,
            }));
        }
    }

    Ok(test_results)
}

fn source_files() -> Result<Vec<(PathBuf, String)>> {
    let current_dir = env::current_dir()?;
    ensure!(
        fs::metadata(current_dir.join("../book.toml")).is_ok(),
        "book.toml not found. make sure to run tests from the correct working directory"
    );
    let book_source_dir = current_dir.join("../src");

    let mut dirs = vec![book_source_dir];
    let mut files = Vec::new();

    while let Some(dir) = dirs.pop() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_type = entry.file_type()?;

            if file_type.is_dir() {
                dirs.push(path);
            } else if file_type.is_file() {
                let content = fs::read_to_string(&path)?;
                files.push((path, content));
            } else {
                bail!("symlinks are not supported");
            }
        }
    }

    Ok(files)
}

fn get_code_blocks(
    path: &Path,
    file_content: &str,
    mut error_sink: impl FnMut(TestError),
) -> Vec<Code> {
    static CODE_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r#"```rteasy(.*)\n([\S\s]*?)```"#).unwrap());

    let mut codes = Vec::new();

    for capture in CODE_REGEX.captures_iter(file_content) {
        let attributes = &capture[1];
        let source = &capture[2];

        let source = fix_code_source(source);
        let kind = match code_kind_from_attributes(attributes) {
            Ok(kind) => kind,
            Err(error) => {
                error_sink(TestError { path: path.to_owned(), source, error });
                continue;
            }
        };

        codes.push(Code { path: path.to_owned(), source, kind });
    }

    codes
}

fn code_kind_from_attributes(attributes: &str) -> Result<CodeKind> {
    let mut kind = CodeKind::Default;
    for attr in attributes.split(',') {
        match attr.trim() {
            "ignore" => {
                ensure!(kind == CodeKind::Default, "conflicting attributes");
                kind = CodeKind::Ignore;
            }
            "no_run" => {
                ensure!(kind == CodeKind::Default, "conflicting attributes");
                kind = CodeKind::NoRun;
            }
            "should_fail" => {
                ensure!(kind == CodeKind::Default, "conflicting attributes");
                kind = CodeKind::ShouldFail;
            }
            "compile_fail" => {
                ensure!(kind == CodeKind::Default, "conflicting attributes");
                kind = CodeKind::CompileFail;
            }
            _ => (),
        }
    }
    Ok(kind)
}

fn fix_code_source(source: &str) -> String {
    let mut result = String::with_capacity(source.len());
    for line in source.lines() {
        if line.trim_start().starts_with('~') {
            let (ws, rest) = line.split_once('~').unwrap();
            result += ws;
            result += rest;
            result += "\n";
        } else {
            result += line;
            result += "\n";
        }
    }
    result
}

fn test_code(code: &Code) -> Result<TestSuccess> {
    match code.kind {
        CodeKind::Default => {
            check_code(&code.source).context("check failed")?;
            run_code(&code.source).context("run failed")?;
        }
        CodeKind::Ignore => return Ok(TestSuccess::Ignored),
        CodeKind::NoRun => {
            check_code(&code.source).context("check failed")?;
        }
        CodeKind::ShouldFail => {
            check_code(&code.source).context("check failed")?;
            run_code(&code.source).err().context("code executed successfully, expected error")?;
        }
        CodeKind::CompileFail => {
            check_code(&code.source).err().context("code compiled successfully, expected error")?;
        }
    }

    Ok(TestSuccess::Passed)
}

fn check_code(source: &str) -> Result<()> {
    let ast = match parser::parse(&source) {
        Ok(ast) => ast,
        Err(e) => bail!(parser::pretty_print_error(&e, &source, None, true)),
    };
    match compiler::check(ast, &Default::default()) {
        Ok(()) => (),
        Err(e) => bail!(e.pretty_print(&source, None, true)),
    };

    Ok(())
}

fn run_code(source: &str) -> Result<()> {
    // Build
    let program = {
        let ast = match parser::parse(&source) {
            Ok(ast) => ast,
            Err(e) => bail!(parser::pretty_print_error(&e, &source, None, true)),
        };

        let backend = compiler_backend_simulator::BackendSimulator;
        match compiler::compile(&backend, (), ast, &Default::default()) {
            Ok(program) => program,
            Err(e) => bail!(e.pretty_print(&source, None, true)),
        }
    };

    // Run max 512 steps
    let mut simulator = simulator::Simulator::init(program);
    for _ in 0..512 {
        let res = simulator.step(false).context("Step failed")?;
        match res {
            Some(res) if matches!(res.kind, simulator::StepResultKind::AssertError) => {
                bail!("assert error")
            }
            Some(_) => (),
            None => break,
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        panic!("use `cargo run` instead");
    }
}
