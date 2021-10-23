#![deny(rust_2018_idioms)]

mod commands;

use ansi_term::Colour::Green;
use anyhow::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rt-easy-cli", about = "rt easy cli")]
pub enum Opt {
    #[structopt(about = "Check the rt file")]
    Check {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },
    #[structopt(about = "Test the rt file")]
    Test {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
        #[structopt(parse(from_os_str))]
        test_file: PathBuf,
    },
}

pub fn run(opt: Opt, ansi_colors: bool) -> Result<String> {
    let msg = match opt {
        Opt::Check { file } => {
            commands::check(file, ansi_colors)?;
            "Code is syntactically valid"
        }
        Opt::Test { file, test_file } => {
            commands::test(file, test_file, ansi_colors)?;
            "Tests passed"
        }
    };

    if ansi_colors {
        Ok(Green.paint(msg).to_string())
    } else {
        Ok(msg.to_string())
    }
}
