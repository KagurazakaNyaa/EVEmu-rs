use clap::Parser;
use internal::config::Config;

mod command;
mod internal;

fn main() {
    let cli = command::index::Cli::parse();
    let confg = Config::build().unwrap();
    cli.run(&confg);
}
