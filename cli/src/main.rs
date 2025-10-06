use clap::Parser;

mod cli;

fn main() {
    let args = cli::Argument::parse();
    dbg!(args);
}
