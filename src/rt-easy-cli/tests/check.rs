use std::path::PathBuf;

#[test]
fn check() {
    let opt = rt_easy_cli::Opt::Check { file: file("mult.rt") };
    rt_easy_cli::run(opt, false).unwrap();
}

#[test]
fn check_invalid01() {
    let opt = rt_easy_cli::Opt::Check { file: file("invalid01.rt") };
    assert!(rt_easy_cli::run(opt, false).is_err());
}

#[test]
fn check_invalid02() {
    let opt = rt_easy_cli::Opt::Check { file: file("invalid02.rt") };
    assert!(rt_easy_cli::run(opt, false).is_err());
}

fn file(name: &str) -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "tests", name].iter().collect()
}
