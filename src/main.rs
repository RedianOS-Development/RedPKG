mod cli;
mod log;

use clap::Parser;

fn main() {
    let args = cli::Cli::parse();
}
