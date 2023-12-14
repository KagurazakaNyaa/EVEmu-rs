use clap::Parser;

mod command;
mod internal;

fn main() {
    let cli = command::index::Cli::parse();
    cli.run();
}
