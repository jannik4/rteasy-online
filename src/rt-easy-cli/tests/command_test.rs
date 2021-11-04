use rt_easy_cli::{Command, Opt};
use std::path::PathBuf;

#[test]
fn test() {
    let opt = Opt {
        no_ansi: true,
        command: Command::Test { file: file("mult.rt"), test_file: file("mult_test.rtt") },
    };
    rt_easy_cli::run(opt).unwrap();
}

fn file(name: &str) -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "tests", name].iter().collect()
}
