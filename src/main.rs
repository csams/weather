use clap::Parser;
use weather::config::{Config, Options};
use weather::run;

fn main() {
    let opts = Options::parse();
    if let Err(s) = Config::build(opts).and_then(run) {
        println!("{}", s);
    }
}
