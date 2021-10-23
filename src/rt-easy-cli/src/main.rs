#![deny(rust_2018_idioms)]

use rt_easy_cli::{run, Opt};
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    match run(opt) {
        Ok(msg) => println!("{}", msg),
        Err(err) => {
            println!("{:?}", err);
            std::process::exit(1);
        }
    }
}
