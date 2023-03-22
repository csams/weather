use clap::Parser;
use weather::config::{Config, Options};
use weather::run;

fn main() {
    let opts = Options::parse();
    if let Err(s) = Config::new(opts).and_then(run) {
        println!("{}", s);
    }
}
