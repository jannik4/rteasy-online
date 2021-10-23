#![deny(rust_2018_idioms)]

mod commands;

use ansi_term::Colour::Green;
use anyhow::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rt-easy-cli", about = "rt easy cli")]
pub struct Opt {
    #[structopt(long, help = "Disable ansi colors")]
    pub no_ansi: bool,
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "rt-easy-cli", about = "rt easy cli")]
pub enum Command {
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

pub fn run(opt: Opt) -> Result<String> {
    let ansi_colors = !opt.no_ansi;
    let msg = match opt.command {
        Command::Check { file } => {
            commands::check(file, ansi_colors)?;
            "Code is syntactically valid"
        }
        Command::Test { file, test_file } => {
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
