use rt_easy_cli::{Command, Opt};
use std::path::PathBuf;

#[test]
fn check() {
    let opt = Opt { no_ansi: true, command: Command::Check { file: file("mult.rt") } };
    rt_easy_cli::run(opt).unwrap();
}

#[test]
fn check_invalid01() {
    let opt = Opt { no_ansi: true, command: Command::Check { file: file("invalid01.rt") } };
    assert!(rt_easy_cli::run(opt).is_err());
}

#[test]
fn check_invalid02() {
    let opt = Opt { no_ansi: true, command: Command::Check { file: file("invalid02.rt") } };
    assert!(rt_easy_cli::run(opt).is_err());
}

fn file(name: &str) -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "tests", name].iter().collect()
}
