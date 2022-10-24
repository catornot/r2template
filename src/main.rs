use lib::Cli;
// use std::env;

mod lib;

fn main() {
    // env::set_var("RUST_BACKTRACE", "0");
    Cli::commands();
}
